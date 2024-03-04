use serde_json::json;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;

use components::persistent_storage::Nvs;
use components::persistent_storage::NvsPartition;
use components::protocomm::transports::httpd::TransportHttpd;
use components::protocomm::*;
use components::wifi::*;

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
    protocomm: WrappedInArcMutex<Protocomm<'a>>,
    wifi_client: WrappedInArcMutex<WifiMgr<'a>>,
    _phantom: PhantomData<&'a ()>, // for compiler to not complain about lifetime parameter
}

impl<'a> WifiProvisioningMgr<'a> {
    pub fn new(
        protocomm: Option<WrappedInArcMutex<Protocomm<'a>>>,
        wifi_client: WrappedInArcMutex<WifiMgr<'a>>,
    ) -> Self {
        let protocomm_new: WrappedInArcMutex<Protocomm>;
        if protocomm.is_none() {
            protocomm_new = Arc::new(Mutex::new(Protocomm::new(
                ProtocomTransport::Httpd(TransportHttpd::new()),
                ProtocommSecurity::Sec0,
            )));
        } else {
            protocomm_new = protocomm.unwrap();
        }
        Self {
            protocomm: protocomm_new,
            wifi_client,
            _phantom: PhantomData,
        }
    }

    pub fn init(&self, config: WifiProvisioningConfig) {
        self.init_ap(config.device_name);
        self.register_listeners();
    }

    pub fn start(&self) -> Result<(), RMakerError> {
        let mut wifi_driv = self.wifi_client.lock().unwrap();
        wifi_driv.set_client_config(WifiClientConfig::default())?;
        wifi_driv.start()?;
        drop(wifi_driv);

        let pc = self.protocomm.lock().unwrap();
        pc.start();

        Ok(())
    }

    pub fn add_endpoint<T>(&self, endpoint: &str, callback: T)
    where
        T: Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'a,
    {
        // todo: look into how idf-c does it and make it transport independent
        let pc = self.protocomm.lock().unwrap();

        pc.register_endpoint(endpoint, callback).unwrap();
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

    fn register_listeners(&self) {
        log::info!("adding provisioning listeners");
        let wifi_driv_prov_config = self.wifi_client.clone();
        let wifi_driv_prov_scan = self.wifi_client.clone();

        let pc = self.protocomm.lock().unwrap();
        pc.set_security_endpoint("prov-session").unwrap(); // hardcoded sec params for sec0

        pc.register_endpoint("proto-ver", proto_ver_callback)
            .unwrap();

        pc.register_endpoint("prov-config", move |ep, data| -> Vec<u8> {
            prov_config_callback(ep, data, wifi_driv_prov_config.to_owned())
        })
        .unwrap();

        pc.register_endpoint("prov-scan", move |ep, data| -> Vec<u8> {
            prov_scan_callback(ep, data, wifi_driv_prov_scan.to_owned())
        })
        .unwrap();
    }

    fn init_ap(&self, device_name: String) {
        let mut wifi_driv = self.wifi_client.lock().unwrap();
        let apconf = WifiApConfig {
            ssid: format!("PROV_{}", device_name),
            ..Default::default()
        };

        wifi_driv.set_ap_config(apconf).unwrap();
    }
}

fn proto_ver_callback(_ep: String, _inp_data: Vec<u8>) -> Vec<u8> {
    let response = json!({
     "prov": {
        "ver": "v1.1",
        "sec_ver" : 0,
        "cap": ["wifi_scan", "no_pop", "no_sec"]
     }
    });

    Vec::from(response.to_string())
}

fn prov_config_callback(
    _ep: String,
    data: Vec<u8>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'_>>,
) -> Vec<u8> {
    log::info!("prov_config called");
    let req_proto = WiFiConfigPayload::decode(&*data).unwrap();

    let msg_type = req_proto.msg();
    let res = match msg_type {
        WiFiConfigMsgType::TypeCmdGetStatus => handle_cmd_get_status(wifi_driv),
        WiFiConfigMsgType::TypeCmdSetConfig => {
            handle_cmd_set_config(req_proto.payload.unwrap(), wifi_driv)
        }
        WiFiConfigMsgType::TypeCmdApplyConfig => handle_cmd_apply_config(),
        _ => unreachable!(),
    };

    res
}

fn prov_scan_callback<'a>(
    _ep: String,
    data: Vec<u8>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'a>>,
) -> Vec<u8> {
    let req_proto = WiFiScanPayload::decode(&*data).unwrap();
    let msg_type = req_proto.msg();

    let res = match msg_type {
        WiFiScanMsgType::TypeCmdScanStart => handle_cmd_scan_start(),
        WiFiScanMsgType::TypeCmdScanStatus => handle_cmd_scan_status(),
        WiFiScanMsgType::TypeCmdScanResult => handle_cmd_scan_result(wifi_driv),
        _ => unreachable!(),
    };

    res
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
