use std::{
    cmp::min,
    collections::HashMap,
    sync::{atomic::AtomicUsize, Mutex},
};

use prost::Message;
use r_maker_claim_payload::Payload;
use rand::rngs::OsRng;
use rcgen::{CertificateParams, DistinguishedName, KeyPair};
use rsa::{pkcs8::EncodePrivateKey, RsaPrivateKey};
use serde_json::json;

use crate::{
    factory::constants::{FACTORY_CERT_KEY, FACTORY_NODE_ID_KEY, FACTORY_PRIV_KEY},
    rmaker_factory,
    wifi_prov::WifiProvisioningMgr,
};

const RMAKER_APP_NAME: &str = "rmaker";
const ASSISTED_CLAIMING_EP: &str = "rmaker_claim";
const CAP_ASSISTED_CLAIM: &str = "claim";

include!(concat!(env!("OUT_DIR"), "/rmaker_claim.rs"));

const MAX_FRAGMENT_LEN: usize = 200;
// A global vector to store data which need to be sent split across multiple requests
static STREAMING_DATA: Mutex<Vec<u8>> = Mutex::new(vec![]);
// For keeping a track of sent CSR bytes
static STREAMING_OFFSET: AtomicUsize = AtomicUsize::new(0);

pub struct AssistedClaiming {}
impl AssistedClaiming {
    pub fn register_ep(prov: &mut WifiProvisioningMgr) {
        prov.add_application(RMAKER_APP_NAME, "1.0", &[CAP_ASSISTED_CLAIM]);
        prov.add_endpoint(
            ASSISTED_CLAIMING_EP,
            Box::new(AssistedClaiming::protocomm_msg_handler),
        );
    }

    fn protocomm_msg_handler(_ep: &str, inp: Vec<u8>) -> Vec<u8> {
        let payload = match RMakerClaimPayload::decode(&*inp) {
            Ok(payload) => payload,
            Err(_) => {
                log::error!("Invalid Assited Claim Payload");
                return vec![];
            }
        };

        let cmd_payoad = match payload.payload.clone().unwrap() {
            Payload::CmdPayload(payload_buf) => payload_buf,
            Payload::RespPayload(_) => {
                log::error!("Invalid cmd payload for claiming.");
                return vec![];
            }
        };

        let mut resp_msg = RMakerClaimPayload::default();
        resp_msg.msg = (payload.msg + 1).into();

        let resp_payload = match RMakerClaimMsgType::try_from(payload.msg).unwrap() {
            RMakerClaimMsgType::TypeCmdClaimStart => {
                AssistedClaiming::handle_claim_start(cmd_payoad)
            }
            RMakerClaimMsgType::TypeCmdClaimInit => AssistedClaiming::handle_claim_init(cmd_payoad),
            RMakerClaimMsgType::TypeCmdClaimVerify => {
                AssistedClaiming::handle_claim_verify(cmd_payoad)
            }
            RMakerClaimMsgType::TypeCmdClaimAbort => todo!(),
            _ => {
                log::error!("Invalid Claim Msg Type.");
                return vec![];
            }
        };

        resp_msg.payload = Some(Payload::RespPayload(resp_payload));

        let resp_vec = resp_msg.encode_to_vec();

        resp_vec
    }

    fn handle_claim_start(_: PayloadBuf) -> RespPayload {
        log::info!("Starting Assisted Claiming");
        let mac_addr = "ABCDEF123456";
        let platform = "esp32c3";
        let resp_string = format!("{{\"mac_addr\": {}, \"platform\": {}}}", mac_addr, platform);

        let mut payload = PayloadBuf::default();
        payload.offset = 0;
        payload.total_len = resp_string.len() as u32;
        payload.payload = resp_string.into();

        let mut response = RespPayload::default();
        response.status = RMakerClaimStatus::Success.into();
        response.buf = Some(payload);

        response
    }

    fn handle_claim_init(inp: PayloadBuf) -> RespPayload {
        let mut response = RespPayload::default();
        response.status = RMakerClaimStatus::Success.into();

        let streaming_offset = STREAMING_OFFSET.load(std::sync::atomic::Ordering::Relaxed);

        if streaming_offset > 0 {
            // CSR already generated, just need to stream it
            let mut streaming_data = STREAMING_DATA.lock().unwrap();
            let total_len = streaming_data.len();

            let remaining_len = min(MAX_FRAGMENT_LEN, total_len - streaming_offset);

            let start_index = streaming_offset;
            let end_index = start_index + remaining_len;

            let data_to_send = streaming_data[start_index..end_index].to_vec();
            if end_index != total_len {
                STREAMING_OFFSET.fetch_add(remaining_len, std::sync::atomic::Ordering::Relaxed);
            } else {
                // Done sending
                STREAMING_OFFSET.store(0, std::sync::atomic::Ordering::Relaxed);
                streaming_data.clear();
            }

            let mut payload_buf = PayloadBuf::default();
            payload_buf.offset = start_index as u32;
            payload_buf.payload = data_to_send;
            payload_buf.total_len = total_len as u32;

            response.buf = Some(payload_buf);
            return response;
        }

        let payload_str = String::from_utf8(inp.payload).unwrap();
        let inp_json: HashMap<String, String> = serde_json::from_str(&payload_str).unwrap();

        let node_id = inp_json.get("node_id").unwrap();
        log::info!("Received node_id: {}", node_id);
        rmaker_factory::set_bytes(FACTORY_NODE_ID_KEY, node_id.as_bytes())
            .expect("Failed to set node id in factory partition");

        let mut rng = OsRng::default();
        let private_key =
            RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate private key");

        let priv_pkcs8_pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .expect("Failed to encode Private Key");

        rmaker_factory::set_bytes(FACTORY_PRIV_KEY, priv_pkcs8_pem.as_bytes())
            .expect("Failed to set private key in factory partitoin");
        log::info!("Successfully saved private key");

        let mut distinguished_name = DistinguishedName::new();
        distinguished_name.push(rcgen::DnType::CommonName, node_id);

        let keypair = KeyPair::from_pem(&priv_pkcs8_pem).expect("Failed to generate KeyPair");
        let mut params = CertificateParams::default();
        params.distinguished_name = distinguished_name;
        let signing_request = params
            .serialize_request(&keypair)
            .expect("Failed to generate certificate signing request")
            .pem()
            .unwrap();

        let resp_json: Vec<u8> = json!({
            "csr": signing_request.trim()
        })
        .to_string()
        .into();

        // Assuming generated CSR will be longer than MAX_FRAGMENT_LEN
        let mut streaming_data = STREAMING_DATA.lock().unwrap();
        *streaming_data = resp_json;
        STREAMING_OFFSET.store(MAX_FRAGMENT_LEN, std::sync::atomic::Ordering::Relaxed);

        let mut payload = PayloadBuf::default();
        payload.offset = 0;
        payload.payload = streaming_data[0..MAX_FRAGMENT_LEN].to_vec();
        payload.total_len = streaming_data.len() as u32;

        response.buf = Some(payload);

        response
    }

    fn handle_claim_verify(inp: PayloadBuf) -> RespPayload {
        let mut response = RespPayload::default();
        response.status = RMakerClaimStatus::Success.into();

        let total_len = inp.total_len as usize;
        let mut streaming_data = STREAMING_DATA.lock().unwrap();
        let remaining_len = total_len - streaming_data.len();
        if remaining_len > 0 {
            streaming_data.extend(inp.payload.iter());
        }
        if remaining_len > MAX_FRAGMENT_LEN {
            return response;
        }

        let recv_json: HashMap<String, String> = serde_json::from_slice(&streaming_data).unwrap();
        streaming_data.clear();
        drop(streaming_data);

        let certificate = recv_json
            .get("certificate")
            .expect("Signed certificate not found in payload");

        rmaker_factory::set_bytes(FACTORY_CERT_KEY, certificate.as_bytes())
            .expect("Failed to save client certificate to factory partition");
        log::info!("Successfully saved client certificate");

        response
    }
}
