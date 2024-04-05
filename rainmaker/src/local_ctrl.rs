use components::protocomm::*;
use crate::node::Node;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::Arc,
};

pub struct LocalCtrlConfig<'a> {
    pub protocom: Protocomm<'a>,
    pub node: Arc<Node<'a>>,
}

impl<'a> LocalCtrlConfig<'a> 
where
    'a : 'static
{
    pub fn local_ctrl_start(&mut self) -> anyhow::Result<(), anyhow::Error> {
        
        let pc = &self.protocom;
        let node = self.node.clone();
        log::info!("adding local_ctrl listeners");

        pc.set_security_endpoint("esp_local_ctrl/session").unwrap();

        pc.register_endpoint("esp_local_ctrl/control", move |ep, data| -> Vec<u8> {
            control_handler(ep, data, node.to_owned())
        })
            .unwrap();

        pc.register_endpoint("esp_local_ctrl/version", version_handler)
            .unwrap();

        pc.start();

        Ok(())
    }
}

pub fn version_handler(
    _ep: String,
    data: Vec<u8>
) -> Vec<u8> {

    let req_proto = LocalCtrlMessage::decode(&*data).unwrap();

    log::info!("local_ctrl_version_payload: {:?}", req_proto);

    "version url Local control version v1.0".as_bytes().to_vec()
}

pub fn control_handler(
    _ep: String,
    data: Vec<u8>,
    node: Arc<Node<'_>>
) -> Vec<u8> {

    let req_proto = LocalCtrlMessage::decode(&*data).unwrap();

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
        LocalCtrlMsgType::TypeCmdSetPropertyValues => handle_cmd_set_property_values(req_proto.payload.unwrap(), node.to_owned()),
        _ => vec![]
    };

    res
}

fn handle_cmd_get_property_count() -> Vec<u8> {
    let mut resp_payload = RespGetPropertyCount::default();
    resp_payload.status = Status::Success.into();
    resp_payload.count = 2;

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

fn handle_cmd_set_property_values(req_payload: local_ctrl_message::Payload, node: Arc<Node<'_>>) -> Vec<u8> {
    let mut resp_payload = RespSetPropertyValues::default();

    match req_payload {
        local_ctrl_message::Payload::CmdSetPropVals(values) => {
            resp_payload.status = Status::Success.into();

            log::info!("{:?}", values);
            log::info!("{:?}", std::str::from_utf8(&values.props[0].value).unwrap());

            let msg = values.props[0].value.clone();

            let received_val: HashMap<String, HashMap<String, Value>> =
                serde_json::from_str(&String::from_utf8(msg).unwrap()).unwrap();
            let devices = received_val.keys();
            for device in devices {
                let params = received_val.get(device).unwrap().to_owned();
                node.exeute_device_callback(&device, params);
            }

            let mut resp = LocalCtrlMessage::default();
            resp.payload = Some(local_ctrl_message::Payload::RespSetPropVals(resp_payload));
            resp.encode_to_vec()
        }
        _ => unreachable!() 
    }
}