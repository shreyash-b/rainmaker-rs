use std::sync::Mutex;

use ed25519_compact::x25519::{KeyPair, PublicKey};

use quick_protobuf::{MessageWrite, Writer};
use sha2::{Digest, Sha256};

use aes::{
    cipher::{KeyIvInit, StreamCipher},
    Aes256,
};
use ctr::Ctr128BE;
use rand::RngCore;

use crate::proto::{
    constants::Status,
    session::{
        sec1::*,
        session::{mod_SessionData, SecSchemeVersion, SessionData},
    },
};

use super::SecurityTrait;

const LOGGER_TAG: &str = "sec1";

type AesCtr = Ctr128BE<Aes256>;

fn debug_u8_arr_as_hex(name: &str, inp: &[u8]) {
    // build a string and append values as log::debug!() prints new line which we don't want
    let mut s = String::from("0x");
    for i in inp {
        s += &format!("{:x}", *i)
    }
    log::debug!(target: LOGGER_TAG, "{name}: {s}");
}

#[derive(Default)]
pub struct Sec1 {
    pub pop: Option<String>,
    pub(crate) sec_data: Mutex<Sec1Data>,
}

#[derive(Default)]
pub(crate) struct Sec1Data {
    pub(crate) random: [u8; 16], // in itialization vector
    device_keypair: Option<KeyPair>,
    client_pubkey: Option<Vec<u8>>,
    cipher: Option<AesCtr>,
}

impl Sec1 {
    fn handle_cmd0(&self, in_proto: SessionCmd0) -> SessionResp0 {
        let client_pub_key = in_proto.client_pubkey;
        let device_ecdh_keypair = KeyPair::generate();
        let client_pub = PublicKey::from_slice(&client_pub_key).unwrap();

        let mut shared_secret = client_pub.dh(&device_ecdh_keypair.sk).unwrap();
        let key_len = shared_secret.len();

        let mut device_random = [0; 16];
        rand::thread_rng().fill_bytes(&mut device_random);

        debug_u8_arr_as_hex("Device random", &device_random);
        debug_u8_arr_as_hex("Shared key(ECDH)", shared_secret.as_ref());

        // XOR SHA256 Hash of PoP with shared secret if application
        match &self.pop {
            None => {}
            Some(pop_value) => {
                let pop_hash = Sha256::digest(pop_value);

                for i in 0..key_len {
                    shared_secret[i] ^= pop_hash[i];
                }

                debug_u8_arr_as_hex("PoP", pop_value.as_bytes());
                debug_u8_arr_as_hex("PoP(SHA256)", &pop_hash);
                debug_u8_arr_as_hex("Shared key(after XOR with PoP)", shared_secret.as_ref());
            }
        }

        let cipher = AesCtr::new(shared_secret.as_ref().into(), device_random.as_ref().into());

        let out_data = SessionResp0 {
            status: Status::Success,
            device_pubkey: device_ecdh_keypair.pk.as_ref().to_vec(),
            device_random: device_random.to_vec(),
        };

        let mut data = self.sec_data.lock().unwrap();

        data.cipher = Some(cipher);
        data.random = device_random;
        data.device_keypair = Some(device_ecdh_keypair);
        data.client_pubkey = Some(client_pub_key);

        drop(data);

        out_data
    }

    fn handle_cmd1(&self, in_proto: SessionCmd1) -> SessionResp1 {
        let mut data = self.sec_data.lock().unwrap();
        let mut client_verify_data = in_proto.client_verify_data;
        let mut client_pubkey = data.client_pubkey.as_ref().unwrap().clone();
        let device_pub_key = data.device_keypair.as_ref().unwrap().pk.as_ref().to_vec();

        let cipher = data.cipher.as_mut().unwrap();
        cipher.apply_keystream(&mut client_verify_data);
        cipher.apply_keystream(&mut client_pubkey);

        debug_u8_arr_as_hex("client verfier", &client_verify_data);

        if client_verify_data == device_pub_key {
            log::debug!(target: LOGGER_TAG, "Client data verfied");
        } else {
            log::debug!(target: LOGGER_TAG, "Could not verify client data");
        }

        drop(data);
        debug_u8_arr_as_hex("sending device verifier", &client_pubkey);

        SessionResp1 {
            status: Status::Success,
            device_verify_data: client_pubkey,
        }
    }
}

impl SecurityTrait for Sec1 {
    fn security_handler(&self, _ep: &str, data: Vec<u8>) -> Vec<u8> {
        let proto_decode = SessionData::try_from(data.as_slice());

        let in_proto = match proto_decode {
            Ok(d) => d,
            Err(_) => {
                // decoding falied
                log::error!("Failed to decode Sec1 payload");
                return vec![];
            }
        };

        if in_proto.sec_ver != SecSchemeVersion::SecScheme1 {
            // incorrect secver
            return vec![];
        }

        let mut out_data = Sec1Payload::default();
        match in_proto.proto {
            mod_SessionData::OneOfproto::sec1(data) => match data.payload {
                mod_Sec1Payload::OneOfpayload::sc0(payload) => {
                    out_data.msg = Sec1MsgType::Session_Response0;
                    out_data.payload =
                        mod_Sec1Payload::OneOfpayload::sr0(self.handle_cmd0(payload));
                }
                mod_Sec1Payload::OneOfpayload::sc1(payload) => {
                    out_data.msg = Sec1MsgType::Session_Response1;
                    out_data.payload =
                        mod_Sec1Payload::OneOfpayload::sr1(self.handle_cmd1(payload));
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        let out_proto = SessionData {
            sec_ver: SecSchemeVersion::SecScheme1,
            proto: mod_SessionData::OneOfproto::sec1(out_data),
        };

        let mut out_vec = vec![];
        let mut writer = Writer::new(&mut out_vec);

        out_proto.write_message(&mut writer).unwrap();

        out_vec
    }

    fn encrypt(&self, input: &mut [u8]) {
        let mut data = self.sec_data.lock().unwrap();
        let cipher = data.cipher.as_mut().unwrap();
        cipher.apply_keystream(input);
    }

    fn decrypt(&self, input: &mut [u8]) {
        self.encrypt(input)
    }
}
