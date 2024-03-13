use components::http::HttpConfiguration;
use serde_json::json;
use std::marker::PhantomData;

use components::persistent_storage::Nvs;
use components::persistent_storage::NvsPartition;
use components::protocomm::*;
use components::wifi::*;

use crate::error::RMakerError;
use crate::utils::{wrap_in_arc_mutex, WrappedInArcMutex};

const PROV_MGR_VER: &str = "v1.1";
const LOGGER_TAH: &str = "wifi_prov";

#[derive(Default)]
pub enum WifiProvScheme {
    #[default]
    SoftAP,
}

#[derive(Default)]
pub struct WifiProvisioningConfig {
    pub device_name: String,
    pub scheme: WifiProvScheme,
    pub security: ProtocommSecurity
}

pub struct WifiProvisioningMgr<'a> {
    protocomm: WrappedInArcMutex<Protocomm<'a>>,
    wifi_client: WrappedInArcMutex<WifiMgr<'static>>,
    device_name: String,
    _phantom: PhantomData<&'a ()>, // for compiler to not complain about lifetime parameter
}

impl<'a> WifiProvisioningMgr<'a> {
    pub fn new(wifi_client: WrappedInArcMutex<WifiMgr<'static>>, config: WifiProvisioningConfig) -> Self {
        let protocomm_config = ProtocommConfig{
            transport: ProtocomTransportConfig::Httpd(HttpConfiguration::default()),
            security: config.security
        };

        let protocomm_new = wrap_in_arc_mutex(Protocomm::new(protocomm_config));

        let mut prov_mgr = Self {
            protocomm: protocomm_new,
            wifi_client,
            device_name: config.device_name,
            _phantom: PhantomData,
        };
        prov_mgr.init();

        prov_mgr

        
    }

    pub fn init(&mut self) {
        let device_name = &self.device_name;
        self.init_ap(device_name);
        self.register_listeners();
    }

    pub fn start(&self) -> Result<(), RMakerError> {
        let mut wifi_driv = self.wifi_client.lock().unwrap();
        log::debug!(target: LOGGER_TAH, "starting wifi in SoftAP mode");
        wifi_driv.set_client_config(WifiClientConfig::default())?;
        wifi_driv.start()?;
        drop(wifi_driv);

        self.print_prov_url();

        Ok(())
    }

    pub fn add_endpoint<T>(&self, endpoint: &str, callback: T)
    where
        T: Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static,
    {
        let pc = self.protocomm.lock().unwrap();

        pc.register_endpoint(endpoint, callback).unwrap();
    }

    pub fn get_provisioned_creds() -> Option<(String, String)> {
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

    fn get_version_info(&self) -> serde_json::Value {

        let ver_info = json!({
            "prov": {
                "ver": PROV_MGR_VER,
                "sec_ver": 1,
                "cap": ["wifi_scan"]
            }
        });
        
        ver_info
    }

    fn register_listeners(&self) {
        log::debug!(target: LOGGER_TAH, "adding provisioning listeners");
        let wifi_driv_prov_config = self.wifi_client.clone();
        let wifi_driv_prov_scan = self.wifi_client.clone();

        let pc = self.protocomm.lock().unwrap();
        pc.set_security_endpoint("prov-session").unwrap(); // hardcoded sec params for sec0

        let version_str = self.get_version_info();
        pc.set_version_endpoint("proto-ver", version_str.to_string()).unwrap();

        pc.register_endpoint("prov-config", move |ep, data| -> Vec<u8> {
            prov_config_callback(ep, data, wifi_driv_prov_config.to_owned())
        })
        .unwrap();

        pc.register_endpoint("prov-scan", move |ep, data| -> Vec<u8> {
            prov_scan_callback(ep, data, wifi_driv_prov_scan.to_owned())
        })
        .unwrap();
    }

    fn init_ap(&self, device_name: &String) {
        let mut wifi_driv = self.wifi_client.lock().unwrap();
        let apconf = WifiApConfig {
            ssid: format!("PROV_{}", device_name),
            ..Default::default()
        };

        wifi_driv.set_ap_config(apconf).unwrap();
    }

    fn print_prov_url(&self) {
        let device_name = &self.device_name;
        let data_json = json!({
            "ver":"v1",
            "name": format!("PROV_{device_name}"),
            "transport":"softap" // becoz no ble
        });

        let qr_url = format!(
            "https://espressif.github.io/esp-jumpstart/qrcode.html?data={}",
            data_json.to_string()
        );

        log::info!(
            "Provisioning started. Visit following url to provision node: {}",
            qr_url
        );
    }
}

fn prov_config_callback(
    _ep: String,
    data: Vec<u8>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'_>>,
) -> Vec<u8> {
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

    resp_msg.encode_to_vec()
}

fn handle_cmd_get_status(wifi_driv: WrappedInArcMutex<WifiMgr<'_>>) -> Vec<u8> {
    let wifi_driv = wifi_driv.lock().unwrap();
    let wifi_client_config = wifi_driv.get_wifi_config().1.unwrap();
    let ip_addr = wifi_driv.get_ip_addr();

    drop(wifi_driv); // no longer needed

    let wifi_state = WifiConnectedState {
        ip4_addr: ip_addr.to_string(),
        auth_mode: components::protocomm::WifiAuthMode::from(wifi_client_config.auth).into(),
        ssid: wifi_client_config.ssid.into(), // to vector
        bssid: vec![],
        channel: 0,
    };

    let mut resp_msg = WiFiConfigPayload::default();
    resp_msg.msg = WiFiConfigMsgType::TypeRespGetStatus.into();

    resp_msg.payload = Some(wi_fi_config_payload::Payload::RespGetStatus(
        RespGetStatus {
            status: Status::Success.into(),
            sta_state: WifiStationState::Connected.into(),
            state: Some(resp_get_status::State::Connected(wifi_state)),
        },
    ));

    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_start() -> Vec<u8> {
    log::info!("Starting wifi scan");
    let mut resp_msg = WiFiScanPayload::default();
    resp_msg.msg = WiFiScanMsgType::TypeRespScanStart.into();
    resp_msg.status = Status::Success.into();

    resp_msg.payload = Some(wi_fi_scan_payload::Payload::RespScanStart(RespScanStart {}));

    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_status() -> Vec<u8> {
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
    log::info!("Sending scan results");

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
