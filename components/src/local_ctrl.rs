use crate::http::*;

use crate::protocomm::*;

pub enum LocalCtrlProtoSec {
    ProtocomSec0 = 0,
    ProtocomSec1,
    ProtocomSec2,
    ProtocomSecCustom,
}

pub struct LocalCtrlProtoSecConfig {
    pub version: u8,
}

pub struct LocalCtrlConfig {
    pub transport: HttpConfiguration,
    pub handlers: LocalCtrlHandlers,
    pub max_properties: u8,
}

// pub struct LocalCtrlTransportConfig {
//     pub httpd: HttpConfiguration,
//     pub ble: BleConfiguration, // To be define
// }

pub struct LocalCtrlHandlers {
    pub get_prop_values: fn(),
    pub set_prop_values: fn(),
}

impl LocalCtrlConfig {
    pub fn local_ctrl_start(&mut self) -> anyhow::Result<(), anyhow::Error> {
    
        let mut server = HttpServer::new(&self.transport).unwrap();
        log::info!("adding local_ctrl listeners");

        server.add_listener(
            "/esp_local_ctrl/".to_string(), 
            HttpMethod::POST, 
            Box::new(local_ctrl_handler)
        );

        server.add_listener(
            "/esp_local_ctrl/session".to_string(), 
            HttpMethod::POST, 
            Box::new(prov_session_callback)
        );
       
        server.add_listener(
            "/esp_local_ctrl/version".to_string(), 
            HttpMethod::POST, 
            Box::new(version_handler)
        );
    
        server.add_listener(
            "/esp_local_ctrl/control".to_string(), 
            HttpMethod::POST, 
            Box::new(control_handler)
        );

        server.listen();
        Ok(())
    }

}

pub fn local_ctrl_handler(_req: HttpRequest) -> HttpResponse {
    HttpResponse::from_bytes("Local Control Started")
}

pub fn version_handler(_req: HttpRequest) -> HttpResponse {
    HttpResponse::from_bytes("version url Local control version v1.0".as_bytes())
}

pub fn session_handler(_req: HttpRequest) -> HttpResponse {
    HttpResponse::from_bytes("Session Established".as_bytes())
}

pub fn control_handler(mut req: HttpRequest) -> HttpResponse {

    let req_data = req.data();

    let req_proto = LocalCtrlMessage::decode(&*req_data).unwrap();

    log::info!("local_ctrl_payload: {:?}", req_proto);

    match req_proto.payload.clone().unwrap() {
        local_ctrl_message::Payload::CmdGetPropCount(values) => {
            println!("values are {:?}", values);
        },
        local_ctrl_message::Payload::CmdGetPropVals(values) => {
            println!("values are {:?}", values);
        },
        local_ctrl_message::Payload::CmdSetPropVals(values) => {
            println!("values are {:?}", values);
        },
        _ => unreachable!(),
    }

    let msg_type = req_proto.msg();

    let res = match msg_type {
        LocalCtrlMsgType::TypeCmdGetPropertyCount => handle_cmd_get_property_count(),
        LocalCtrlMsgType::TypeCmdGetPropertyValues => handle_cmd_get_property_values(req_proto.payload.unwrap()),
        LocalCtrlMsgType::TypeCmdSetPropertyValues => handle_cmd_set_property_values(req_proto.payload.unwrap()),
        _ => vec![]
    };

    HttpResponse::from_bytes(&*res)
}

fn handle_cmd_get_property_count() -> Vec<u8> {
    let mut resp_payload = RespGetPropertyCount::default();
    resp_payload.status = Status::Success.into();
    resp_payload.count = 1;

    let mut resp = LocalCtrlMessage::default();
    resp.payload = Some(local_ctrl_message::Payload::RespGetPropCount(resp_payload));
    resp.encode_to_vec()
}

fn handle_cmd_get_property_values(req_payload: local_ctrl_message::Payload) -> Vec<u8> {
    let mut resp_payload = RespGetPropertyValues::default();

    match req_payload {
        local_ctrl_message::Payload::CmdGetPropVals(values) => {
            resp_payload.status = Status::Success.into();

            log::info!("{:?}", values.indices);
            for i in values.indices {
                let mut prop_info = PropertyInfo::default();
                prop_info.name = "Power".to_string();
                prop_info.r#type = 2;
                prop_info.flags = 0;
                prop_info.value = vec![0];
                log::info!("Get Property {} : {:?}", i, prop_info);
                resp_payload.props.push(prop_info);
            }

            let mut resp = LocalCtrlMessage::default();
            resp.payload = Some(local_ctrl_message::Payload::RespGetPropVals(resp_payload));
            resp.encode_to_vec()
        },
        _ => unreachable!()
    }
    
}

fn handle_cmd_set_property_values(req_payload: local_ctrl_message::Payload) -> Vec<u8> {
    let mut resp_payload = RespSetPropertyValues::default();

    match req_payload {
        local_ctrl_message::Payload::CmdSetPropVals(values) => {
            resp_payload.status = Status::Success.into();

            log::info!("{:?}", values);
            log::info!("{:?}", std::str::from_utf8(&values.props[0].value).unwrap());

            let mut resp = LocalCtrlMessage::default();
            resp.payload = Some(local_ctrl_message::Payload::RespSetPropVals(resp_payload));
            resp.encode_to_vec()
        }
        _ => unreachable!() 
    }
}

pub(crate) fn prov_session_callback(mut _req: HttpRequest) -> HttpResponse {
    let mut res_proto = SessionData::default();
    res_proto.set_sec_ver(SecSchemeVersion::SecScheme0);
    res_proto.proto = Some(session_data::Proto::Sec0(Sec0Payload {
        msg: Sec0MsgType::S0SessionResponse.into(),
        payload: Some(sec0_payload::Payload::Sr(S0SessionResp {
            status: Status::Success.into(),
        })),
    }));

    let res_data = res_proto.encode_to_vec();

    HttpResponse::from_bytes(res_data)
}