use prost::Message;

use crate::protocomm::proto::*;

use super::SecurityTrait;

#[derive(Debug)]
pub struct Sec0;

impl SecurityTrait for Sec0{
    type SecParams = ();

    fn security_handler(_ep_name: String,_data: Vec<u8>) -> Vec<u8> {
        let mut resp_payload = Sec0Payload::default();
        resp_payload.msg = Sec0MsgType::S0SessionResponse.into();
        resp_payload.payload = Some(sec0_payload::Payload::Sr(S0SessionResp{status: Status::Success.into()}));

        let mut resp = SessionData::default();
        resp.sec_ver = SecSchemeVersion::SecScheme0.into();
        resp.proto = Some(session_data::Proto::Sec0(resp_payload));

        resp.encode_to_vec()
    }
    
    fn encrypt(indata: Vec<u8>, _sec_params: Self::SecParams) -> Vec<u8> {
        // Sec0 has no security
        indata
    }
    
    fn decrypt(indata: Vec<u8>, _sec_params: Self::SecParams) -> Vec<u8> {
        // Sec0 has no security
        indata
    }
    
    
    
}