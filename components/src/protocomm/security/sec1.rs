use prost::Message;
use ed25519_compact::x25519::{KeyPair, PublicKey};
use crate::protocomm::{sec1_payload, session_data, Sec1MsgType, Sec1Payload, SecSchemeVersion, SessionCmd0, SessionCmd1, SessionData, SessionResp0, SessionResp1, Status};

use sha2::{Digest, Sha256};

use aes::{cipher::{KeyIvInit, StreamCipher}, Aes256};
use ctr::Ctr128BE;
use rand::RngCore;

use super::SecurityTrait;

const LOGGER_TAG: &str = "sec1";

type AesCtr = Ctr128BE<Aes256>;

fn debug_u8_arr_as_hex(name: &str, inp: &[u8]){
    // build a string and append values as log::debug!() prints new line which we don't want
    let mut s = String::from("0x");
    for i in inp{
        s += &format!("{:x}", *i)
    }
    log::debug!(target: LOGGER_TAG, "{name}: {s}");
}

#[derive(Default)]
pub struct Sec1{
    pub pop: Option<String>,
    pub(crate) sec_data: Sec1Data
}

#[derive(Default)]
pub(crate) struct Sec1Data{
    pub(crate) random: [u8; 16], // in itialization vector
    device_keypair: Option<KeyPair>,
    client_pubkey: Option<Vec<u8>>,
    cipher: Option<AesCtr>
}

impl Sec1{
    fn handle_cmd0(&mut self, in_proto: SessionCmd0) -> SessionResp0 {
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
        match &self.pop{
            None => {},
            Some(pop_value) => {
                let pop_hash = Sha256::digest(&pop_value);
    
                for i in 0..key_len{
                    shared_secret[i] ^= pop_hash[i];
                }
    
                debug_u8_arr_as_hex("PoP", pop_value.as_bytes());
                debug_u8_arr_as_hex("PoP(SHA256)", &pop_hash);
                debug_u8_arr_as_hex("Shared key(after XOR with PoP)", shared_secret.as_ref());

            }
        }
        
        let cipher = AesCtr::new(shared_secret.as_ref().into(), device_random.as_ref().into());
        
        let mut out_data = SessionResp0::default();
        out_data.status = Status::Success.into();
        out_data.device_pubkey = device_ecdh_keypair.pk.as_ref().to_vec();
        out_data.device_random = device_random.to_vec();

        self.sec_data.cipher = Some(cipher);
        self.sec_data.random = device_random;
        self.sec_data.device_keypair = Some(device_ecdh_keypair);
        self.sec_data.client_pubkey = Some(client_pub_key);

        out_data
    }

    fn handle_cmd1(&mut self, in_proto: SessionCmd1) -> SessionResp1{
        let mut client_verify_data = in_proto.client_verify_data;
        let device_pub_key = self.sec_data.device_keypair.as_ref().unwrap().pk.as_ref();

        let cipher = self.sec_data.cipher.as_mut().unwrap();
        cipher.apply_keystream(&mut client_verify_data);

        debug_u8_arr_as_hex("client verfier", &client_verify_data);

        if client_verify_data == device_pub_key {
            log::debug!(target: LOGGER_TAG, "Client data verfied");
        } else {
            log::debug!(target: LOGGER_TAG, "Could not verify client data");
        }

        let mut client_pubkey = self.sec_data.client_pubkey.as_ref().unwrap().clone(); // remove this clone later
        cipher.apply_keystream(&mut client_pubkey);
        debug_u8_arr_as_hex("sending device verifier", &client_pubkey);

        let mut out_data = SessionResp1::default();
        out_data.status = Status::Success.into();
        out_data.device_verify_data = client_pubkey;

        out_data
    }
    
}

impl SecurityTrait for Sec1{
    fn security_handler(&mut self, _ep: String, data: Vec<u8>) -> Vec<u8> {
        let proto_decode = SessionData::decode(data.as_ref());
    
        let in_proto = match proto_decode{
            Ok(d) => {
                d
            },
            Err(_) => {
                // decoding falied
                return vec![]
            }
        };
    
        if in_proto.sec_ver != i32::from(SecSchemeVersion::SecScheme1){
            // incorrect secver
            return vec![]
        }
    
        let mut out_data = Sec1Payload::default();
        match in_proto.proto.unwrap() {
            session_data::Proto::Sec1(data) => {
                match data.payload.unwrap(){
                    sec1_payload::Payload::Sc0(payload) => {
                        out_data.msg = Sec1MsgType::SessionResponse0.into();
                        out_data.payload = Some(sec1_payload::Payload::Sr0(self.handle_cmd0(payload)));
                    },
                    sec1_payload::Payload::Sc1(payload) => {
                        out_data.msg = Sec1MsgType::SessionResponse1.into();
                        out_data.payload = Some(sec1_payload::Payload::Sr1(self.handle_cmd1(payload)));
                    },
                    _ => unreachable!()
                }  
            },
            _ => unreachable!(),
        };

        let mut out_proto = SessionData::default();
        out_proto.sec_ver = SecSchemeVersion::SecScheme1.into();
        out_proto.proto = Some(session_data::Proto::Sec1(out_data));

        out_proto.encode_to_vec()
    }

    fn encrypt(&mut self, input: &mut [u8]) {
        let cipher = self.sec_data.cipher.as_mut().unwrap();
        cipher.apply_keystream(input);

    }

    fn decrypt(&mut self, input: &mut [u8])  {
        self.encrypt(input)
    }
}
