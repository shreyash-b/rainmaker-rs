use prost::Message;

use crate::protocomm::proto::*;

use super::SecurityTrait;

#[derive(Default, Debug)]
pub struct Sec0;

impl SecurityTrait for Sec0 {
    fn security_handler(&mut self, _ep_name: String, _data: Vec<u8>) -> Vec<u8> {
        let resp_payload = Sec0Payload {
            msg: Sec0MsgType::S0SessionResponse.into(),
            payload: Some(sec0_payload::Payload::Sr(S0SessionResp {
                status: Status::Success.into(),
            })),
        };

        let resp = SessionData {
            sec_ver: SecSchemeVersion::SecScheme0.into(),
            proto: Some(session_data::Proto::Sec0(resp_payload)),
        };

        resp.encode_to_vec()
    }

    fn encrypt(&mut self, _indata: &mut [u8]) {}

    fn decrypt(&mut self, _indata: &mut [u8]) {}
}
