use quick_protobuf::{MessageWrite, Writer};

use crate::proto::{
    constants::Status,
    session::{sec0::*, session::*},
};

use super::SecurityTrait;

#[derive(Default, Debug)]
pub struct Sec0;

impl SecurityTrait for Sec0 {
    fn security_handler(&self, _ep_name: &str, _data: Vec<u8>) -> Vec<u8> {
        let mut resp_vec = Vec::default();
        let mut writer = Writer::new(&mut resp_vec);

        let resp_payload = Sec0Payload {
            msg: Sec0MsgType::S0_Session_Response,
            payload: mod_Sec0Payload::OneOfpayload::sr(S0SessionResp {
                status: Status::Success,
            }),
        };

        let resp = SessionData {
            sec_ver: SecSchemeVersion::SecScheme0,
            proto: mod_SessionData::OneOfproto::sec0(resp_payload),
        };

        resp.write_message(&mut writer).unwrap();
        resp_vec
    }

    fn encrypt(&self, _indata: &mut [u8]) {}

    fn decrypt(&self, _indata: &mut [u8]) {}
}
