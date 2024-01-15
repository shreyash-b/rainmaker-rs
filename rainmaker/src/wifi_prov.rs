#[cfg(target_os = "espidf")]
use components::wifi::WifiMgr;
use components::{
    http::{HttpConfiguration, HttpRequest, HttpResponse, HttpServer},
    protocomm::*,
};
use serde_json::json;
use prost::Message;

include!(concat!(env!("OUT_DIR"), "/rainmaker.rs"));

pub fn prov_test() {
    #[cfg(target_os = "espidf")]
    let mut wifi = WifiMgr::new();

    #[cfg(target_os = "espidf")]
    wifi.set_softap();

    #[cfg(target_os = "espidf")]
    wifi.set_client("Connecting...", "0000@1111");

    #[cfg(target_os = "espidf")]
    wifi.start();

    let mut config = HttpConfiguration::default();

    if cfg!(target_os = "espidf") || true {
        log::info!("running on esp... changing http server port to 80");
        config.port = 80;
    }

    let mut server = HttpServer::new(&config).unwrap();

    server.add_listener(
        "/proto-ver",
        components::http::HttpMethod::POST,
        Box::new(proto_ver_callback),
    );
    server.add_listener(
        "/prov-session",
        components::http::HttpMethod::POST,
        Box::new(prov_session_callback),
    );
    server.add_listener(
        "/prov-config",
        components::http::HttpMethod::POST,
        Box::new(prov_config_callback),
    );
    server.add_listener(
        "/prov-scan",
        components::http::HttpMethod::POST,
        Box::new(prov_scan_callback),
    );

    server.listen().unwrap();
}

fn proto_ver_callback(_req: HttpRequest) -> HttpResponse {
    let response = json!({
     "prov": {
        "ver": "v1.1",
        "sec_ver" : 0,
        "cap": ["wifi_scan", "no_pop", "no_sec"]
     }
    });
    HttpResponse::from_bytes(Vec::from(response.to_string()))
}

fn prov_session_callback(mut _req: HttpRequest) -> HttpResponse {
    log::info!(
        "received: {:?}",
        SessionData::decode(&*_req.data()).unwrap()
    );

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

fn prov_config_callback(mut req: HttpRequest) -> HttpResponse {
    let req_data = req.data();
    let req_proto = WiFiConfigPayload::decode(&*req_data).unwrap();

    let msg_type = req_proto.msg();
    let res = match msg_type {
        WiFiConfigMsgType::TypeCmdGetStatus => handle_cmd_get_status(),
        WiFiConfigMsgType::TypeCmdSetConfig => handle_cmd_set_config(req_proto.payload.unwrap()),
        WiFiConfigMsgType::TypeCmdApplyConfig => handle_cmd_apply_config(),
        _ => unreachable!(),
    };
    HttpResponse::from_bytes(&*res)
}

fn prov_scan_callback(mut req: HttpRequest) -> HttpResponse {
    let req_data = req.data();
    let req_proto = WiFiScanPayload::decode(&*req_data).unwrap();
    let msg_type = req_proto.msg();

    let res = match msg_type {
        WiFiScanMsgType::TypeCmdScanStart => handle_cmd_scan_start(),
        WiFiScanMsgType::TypeCmdScanStatus => handle_cmd_scan_status(),
        WiFiScanMsgType::TypeCmdScanResult => handle_cmd_scan_result(),
        _ => unreachable!(),
    };

    HttpResponse::from_bytes(&*res)
}


fn handle_cmd_set_config(req_payload: wi_fi_config_payload::Payload) -> Vec<u8> {
    match req_payload {
        wi_fi_config_payload::Payload::CmdSetConfig(config) => {
            let ssid = String::from_utf8(config.ssid).unwrap();
            let paraphrase = String::from_utf8(config.passphrase).unwrap();
            log::info!(
                "received wifi-credentials : ssid={}, paraphrase={}",
                ssid,
                paraphrase
            );

            let mut res_data = WiFiConfigPayload::default();
            res_data.msg = WiFiConfigMsgType::TypeRespSetConfig.into();
            res_data.payload = Some(wi_fi_config_payload::Payload::RespSetConfig(
                RespSetConfig {
                    status: Status::Success.into(),
                },
            ));

            res_data.encode_to_vec()
        }
        _ => unreachable!(),
    }
}

fn handle_cmd_apply_config() -> Vec<u8> {
    let mut resp_msg = WiFiConfigPayload::default();
    resp_msg.msg = WiFiConfigMsgType::TypeRespApplyConfig.into();

    resp_msg.payload = Some(wi_fi_config_payload::Payload::RespApplyConfig(
        RespApplyConfig {
            status: Status::Success.into(),
        },
    ));

    log::info!("let's say that wifi config is applied");
    resp_msg.encode_to_vec()
}

fn handle_cmd_get_status() -> Vec<u8> {
    let mut resp_msg = WiFiConfigPayload::default();
    resp_msg.msg = WiFiConfigMsgType::TypeRespGetStatus.into();

    resp_msg.payload = Some(wi_fi_config_payload::Payload::RespGetStatus(
        RespGetStatus {
            status: Status::Success.into(),
            sta_state: WifiStationState::Connected.into(),
            state: Some(resp_get_status::State::Connected(WifiConnectedState {
                ip4_addr: "192.168.15.15".to_string(),
                auth_mode: WifiAuthMode::Open.into(),
                ssid: String::from("dummy_connected_wifi").encode_to_vec(),
                bssid: vec![],
                channel: 5,
            })),
        },
    ));

    log::info!(
        "let's fake current state of wifi as: {:?}",
        resp_msg.payload
    );
    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_start() -> Vec<u8> {
    log::info!("starting wifi scan");
    let mut resp_msg = WiFiScanPayload::default();
    resp_msg.msg = WiFiScanMsgType::TypeRespScanStart.into();
    resp_msg.status = Status::Success.into();

    resp_msg.payload = Some(wi_fi_scan_payload::Payload::RespScanStart(RespScanStart {}));

    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_status() -> Vec<u8> {
    log::info!("sending wifi scan status");
    let mut resp_msg = WiFiScanPayload::default();
    resp_msg.msg = WiFiScanMsgType::TypeRespScanStatus.into();
    resp_msg.status = Status::Success.into();

    resp_msg.payload = Some(wi_fi_scan_payload::Payload::RespScanStatus(
        RespScanStatus {
            scan_finished: true,
            result_count: 1,
        },
    ));

    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_result() -> Vec<u8> {
    log::info!("sending scan result");
    let mut resp_msg = WiFiScanPayload::default();
    resp_msg.msg = WiFiScanMsgType::TypeRespScanResult.into();
    resp_msg.status = Status::Success.into();

    let mut dummy_config = WiFiScanResult::default();
    dummy_config.ssid = "DOES_NOT_EXIST".as_bytes().into();

    resp_msg.payload = Some(wi_fi_scan_payload::Payload::RespScanResult(
        RespScanResult {
            entries: vec![dummy_config],
        },
    ));

    resp_msg.encode_to_vec()
}
