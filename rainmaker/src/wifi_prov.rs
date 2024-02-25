use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;

use components::http::*;
use components::persistent_storage::Nvs;
use components::persistent_storage::NvsPartition;
use components::protocomm::*;
use components::wifi::*;
use serde_json::json;

use crate::error::RMakerError;

type WrappedInArcMutex<T> = Arc<Mutex<T>>;

#[derive(Default)]
pub enum WifiProvScheme {
    #[default]
    SoftAP,
}

#[derive(Default)]
pub struct WifiProvisioningConfig {
    pub device_name: String,
    pub scheme: WifiProvScheme,
}

pub struct WifiProvisioningMgr<'a> {
    http_server: WrappedInArcMutex<HttpServer<'a>>,
    wifi_client: WrappedInArcMutex<WifiMgr<'a>>,
    _phantom: PhantomData<&'a ()>, // for compiler to not complain about lifetime parameter
}

impl<'a> WifiProvisioningMgr<'a> {
    pub fn new(
        http_server: WrappedInArcMutex<HttpServer<'a>>,
        wifi_client: WrappedInArcMutex<WifiMgr<'a>>,
    ) -> Self {
        Self {
            http_server,
            wifi_client,
            _phantom: PhantomData,
        }
    }

    pub fn init(&mut self, config: WifiProvisioningConfig) {
        self.init_ap(config.device_name);
        self.add_listeners();
    }

    pub fn start(&self) -> Result<(), RMakerError> {
        let http_server = self.http_server.lock().unwrap();

        let mut wifi_driv = self.wifi_client.lock().unwrap();
        wifi_driv.set_client_config(WifiClientConfig::default())?;
        wifi_driv.start()?;
        drop(wifi_driv);

        http_server.listen();

        Ok(())
    }

    pub fn add_endpoint(
        &self,
        endpoint: String,
        callback: Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>,
    ) {
        // todo: look into how idf-c does it and make it transport independent
        let mut http_server = self.http_server.lock().unwrap();
        http_server.add_listener(format!("/{}", endpoint), HttpMethod::POST, callback)
    }

    pub fn get_provisioned_creds(&self) -> Option<(String, String)> {
        let nvs = Nvs::new(NvsPartition::new("nvs").unwrap(), "wifi_creds").unwrap();
        let ssid_nvs = nvs.get_bytes("ssid");
        if ssid_nvs == None {
            None
        } else {
            let ssid = String::from_utf8(ssid_nvs.unwrap()).unwrap();
            let password = String::from_utf8(nvs.get_bytes("password").unwrap()).unwrap(); // test for empty password

            Some((ssid, password))
        }
    }

    fn add_listeners(&mut self) {
        log::info!("adding provisioning listeners");
        let mut http_server = self.http_server.lock().unwrap();
        let wifi_driv_prov_config = self.wifi_client.clone();
        let wifi_driv_prov_scan = self.wifi_client.clone();

        http_server.add_listener(
            "/proto-ver".to_string(),
            HttpMethod::POST,
            Box::new(proto_ver_callback),
        );

        http_server.add_listener(
            "/prov-session".to_string(),
            HttpMethod::POST,
            Box::new(prov_session_callback),
        );

        http_server.add_listener(
            "/prov-config".to_string(),
            HttpMethod::POST,
            Box::new(move |req| -> HttpResponse {
                prov_config_callback(req, wifi_driv_prov_config.clone())
            }),
        );

        http_server.add_listener(
            "/prov-scan".to_string(),
            HttpMethod::POST,
            Box::new(move |req| -> HttpResponse {
                prov_scan_callback(req, wifi_driv_prov_scan.clone())
            }),
        );
    }

    fn init_ap(&mut self, device_name: String) {
        let mut wifi_driv = self.wifi_client.lock().unwrap();
        let apconf = WifiApConfig {
            ssid: format!("PROV_{}", device_name),
            ..Default::default()
        };

        wifi_driv.set_ap_config(apconf).unwrap();
    }
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

fn prov_config_callback(
    mut req: HttpRequest,
    wifi_driv: WrappedInArcMutex<WifiMgr<'_>>,
) -> HttpResponse {
    log::info!("prov_config called");
    let req_data = req.data();
    let req_proto = WiFiConfigPayload::decode(&*req_data).unwrap();

    log::info!("prov_config: {:?}", req_proto);

    let msg_type = req_proto.msg();
    let res = match msg_type {
        WiFiConfigMsgType::TypeCmdGetStatus => handle_cmd_get_status(wifi_driv),
        WiFiConfigMsgType::TypeCmdSetConfig => {
            handle_cmd_set_config(req_proto.payload.unwrap(), wifi_driv)
        }
        WiFiConfigMsgType::TypeCmdApplyConfig => handle_cmd_apply_config(),
        _ => unreachable!(),
    };
    HttpResponse::from_bytes(&*res)
}

fn prov_scan_callback<'a>(
    mut req: HttpRequest,
    wifi_driv: WrappedInArcMutex<WifiMgr<'a>>,
) -> HttpResponse {
    let req_data = req.data();
    let req_proto = WiFiScanPayload::decode(&*req_data).unwrap();
    let msg_type = req_proto.msg();

    let res = match msg_type {
        WiFiScanMsgType::TypeCmdScanStart => handle_cmd_scan_start(),
        WiFiScanMsgType::TypeCmdScanStatus => handle_cmd_scan_status(),
        WiFiScanMsgType::TypeCmdScanResult => handle_cmd_scan_result(wifi_driv),
        _ => unreachable!(),
    };

    HttpResponse::from_bytes(&*res)
}

fn handle_cmd_set_config(
    req_payload: wi_fi_config_payload::Payload,
    wifi_driv: WrappedInArcMutex<WifiMgr<'_>>,
) -> Vec<u8> {
    let mut wifi_driv = wifi_driv.lock().unwrap();
    match req_payload {
        wi_fi_config_payload::Payload::CmdSetConfig(config) => {
            let wifi_client_config = WifiClientConfig {
                ssid: String::from_utf8(config.ssid.clone()).unwrap(),
                password: String::from_utf8(config.passphrase.clone()).unwrap(),
                bssid: config.bssid,
                ..Default::default()
            };
            wifi_driv.set_client_config(wifi_client_config).unwrap();
            wifi_driv.connect().unwrap(); // so that esp32 crashes incase wifi creds are wrong. Not the best solution, i know.

            let nvs_partition = NvsPartition::new("nvs").unwrap();
            let mut nvs = Nvs::new(nvs_partition, "wifi_creds").unwrap();

            nvs.set_bytes("ssid", config.ssid.as_ref()).unwrap();
            nvs.set_bytes("password", config.passphrase.as_ref())
                .unwrap();

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

fn handle_cmd_get_status(_wifi_driv: WrappedInArcMutex<WifiMgr<'_>>) -> Vec<u8> {
    // let wifi_driv = wifi_driv.lock().unwrap();
    // let wifi_config = wifi_driv.get_wifi_config();

    let mut resp_msg = WiFiConfigPayload::default();
    resp_msg.msg = WiFiConfigMsgType::TypeRespGetStatus.into();

    resp_msg.payload = Some(wi_fi_config_payload::Payload::RespGetStatus(
        RespGetStatus {
            status: Status::Success.into(),
            sta_state: WifiStationState::Connected.into(),
            state: Some(resp_get_status::State::Connected(WifiConnectedState {
                ip4_addr: "192.168.15.15".to_string(),
                auth_mode: components::protocomm::WifiAuthMode::Open.into(),
                ssid: String::from("dummy_connected_wifi").encode_to_vec(),
                bssid: vec![],
                channel: 5,
            })),
        },
    ));

    log::info!("faking current state of wifi");
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

fn handle_cmd_scan_result(wifi_driv: WrappedInArcMutex<WifiMgr<'_>>) -> Vec<u8> {
    log::info!("sending scan result");

    let mut resp_msg = WiFiScanPayload::default();
    resp_msg.msg = WiFiScanMsgType::TypeRespScanResult.into();
    resp_msg.status = Status::Success.into();

    let mut wifi_driv = wifi_driv.lock().unwrap();
    let wifi_networks = wifi_driv.scan().unwrap();
    drop(wifi_driv);
    let mut scan_results = Vec::<WiFiScanResult>::new();

    for entry in wifi_networks {
        scan_results.push(WiFiScanResult {
            ssid: entry.ssid.as_bytes().to_vec(),
            auth: components::protocomm::WifiAuthMode::from(entry.auth).into(),
            bssid: entry.bssid,
            channel: entry.channel.into(),
            ..Default::default()
        })
    }

    resp_msg.payload = Some(wi_fi_scan_payload::Payload::RespScanResult(
        RespScanResult {
            entries: scan_results,
        },
    ));

    resp_msg.encode_to_vec()
}
