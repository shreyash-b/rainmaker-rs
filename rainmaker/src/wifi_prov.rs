use components::persistent_storage::Nvs;
use components::persistent_storage::NvsPartition;
use components::protocomm::*;
use components::wifi::*;
use serde_json::json;
use transports::ble::TransportBleConfig;

use crate::error::RMakerError;
use crate::utils::wrap_in_arc_mutex;
use crate::utils::WrappedInArcMutex;

const PROV_MGR_VER: &str = "v1.1";
const LOGGER_TAH: &str = "wifi_prov";
const CAP_WIFI_SCAN: &str = "wifi_scan"; // wifi scan capability
const CAP_NO_SEC: &str = "no_sec"; // capability signifying sec0
const CAP_NO_POP: &str = "no_pop"; // no PoP in case of sec1 and sec2

#[derive(Debug, Default)]
pub enum WifiProvScheme {
    SoftAP,
    #[default]
    Ble,
}

#[derive(Default)]
pub struct WifiProvisioningConfig {
    pub device_name: String,
    pub scheme: WifiProvScheme,
    pub security: ProtocommSecurity,
}

pub struct WifiProvisioningMgr {
    protocomm: Protocomm,
    wifi_client: WrappedInArcMutex<WifiMgr<'static>>,
    device_name: String,
    version_string: String,
    nvs_partition: NvsPartition,
    scheme: WifiProvScheme,
}

impl WifiProvisioningMgr {
    pub fn new(
        wifi_client: WrappedInArcMutex<WifiMgr<'static>>,
        config: WifiProvisioningConfig,
        nvs_partition: NvsPartition,
    ) -> Self {
        let version_info = Self::get_version_info(&config.security);
        let device_name = format!("PROV_{}", config.device_name.to_uppercase());
        let protocomm_config = ProtocommConfig {
            transport: match &config.scheme {
                WifiProvScheme::SoftAP => ProtocomTransportConfig::Httpd(Default::default()),
                WifiProvScheme::Ble => ProtocomTransportConfig::Ble(TransportBleConfig {
                    device_name: device_name.clone(),
                    ..Default::default()
                }),
            },
            security: config.security,
        };

        let protocomm = Protocomm::new(protocomm_config);

        Self {
            protocomm,
            wifi_client,
            device_name,
            scheme: config.scheme,
            version_string: version_info,
            nvs_partition,
        }
    }

    pub fn wrap(
        wifi_client: WifiMgr<'static>,
        config: WifiProvisioningConfig,
        nvs_partition: NvsPartition,
    ) -> Self {
        Self::new(wrap_in_arc_mutex(wifi_client), config, nvs_partition)
    }

    pub fn init(&mut self) {
        match self.scheme {
            WifiProvScheme::SoftAP => {
                self.init_ap();
            }
            WifiProvScheme::Ble => {}
        }
        self.register_listeners(self.version_string.clone());
    }

    pub fn start(&mut self) -> Result<(), RMakerError> {
        self.init();

        let mut wifi_driv = self.wifi_client.lock().unwrap();
        log::debug!(target: LOGGER_TAH, "starting wifi in SoftAP mode");
        wifi_driv.set_client_config(WifiClientConfig::default())?;
        wifi_driv.start()?;
        drop(wifi_driv);
        self.protocomm.start();

        self.print_prov_url();

        Ok(())
    }

    pub fn add_endpoint(&mut self, endpoint: &str, callback: ProtocommCallbackType) {
        let pc = &mut self.protocomm;

        pc.register_endpoint(endpoint, callback).unwrap();
    }

    pub fn get_provisioned_creds(nvs_partition: NvsPartition) -> Option<(String, String)> {
        let nvs = Nvs::new(nvs_partition, "wifi_creds").unwrap();
        let ssid_nvs = nvs.get_bytes("ssid");
        if ssid_nvs.is_none() {
            None
        } else {
            let ssid = String::from_utf8(ssid_nvs.unwrap()).unwrap();
            let password = String::from_utf8(nvs.get_bytes("password").unwrap()).unwrap(); // test for empty password

            Some((ssid, password))
        }
    }

    pub fn reset_wifi_provisioning(nvs_partition: NvsPartition) -> Result<(), RMakerError> {
        log::warn!("Resetting WiFi Provisioned Credentials");
        if Self::get_provisioned_creds(nvs_partition.clone()).is_none() {
            log::error!("Abort. WiFi not provisioned");
            return Err(RMakerError("not provisioned".to_string()));
        }

        let mut namespace = Nvs::new(nvs_partition, "wifi_creds")?;
        namespace.remove("ssid")?;
        namespace.remove("password")?;
        Ok(())
    }

    pub fn connect(&mut self) -> Result<(), RMakerError> {
        if let Some((ssid, pass)) = Self::get_provisioned_creds(self.nvs_partition.clone()) {
            let mut wifi = self.wifi_client.lock().unwrap();
            wifi.set_client_config(WifiClientConfig {
                ssid,
                password: pass,
                ..Default::default()
            })?;
            wifi.start()?;
            wifi.assured_connect();
        }

        Ok(())
    }

    fn get_version_info(sec_config: &ProtocommSecurity) -> String {
        let mut wifi_capabilities = vec![CAP_WIFI_SCAN];
        let sec_ver = match sec_config {
            ProtocommSecurity::Sec0(_) => {
                wifi_capabilities.push(CAP_NO_SEC);
                // return sec0
                0
            }
            ProtocommSecurity::Sec1(sec1_inner) => {
                if sec1_inner.pop.is_none() {
                    wifi_capabilities.push(CAP_NO_POP)
                };
                // return sec1
                1
            }
        };

        let ver_info = json!({
            "prov": {
                "ver": PROV_MGR_VER,
                "sec_ver": sec_ver,
                "cap": wifi_capabilities
            }
        });

        ver_info.to_string()
    }

    fn register_listeners(&mut self, version_info: String) {
        log::debug!(target: LOGGER_TAH, "adding provisioning listeners");
        let wifi_driv_prov_config = self.wifi_client.clone();
        let wifi_driv_prov_scan = self.wifi_client.clone();
        let nvs_partition = self.nvs_partition.clone();

        let pc = &mut self.protocomm;
        pc.set_security_endpoint("prov-session").unwrap(); // hardcoded sec params for sec0

        pc.set_version_endpoint("proto-ver", version_info.to_string())
            .unwrap();

        pc.register_endpoint(
            "prov-config",
            Box::new(move |ep, data| -> Vec<u8> {
                prov_config_callback(
                    ep,
                    data,
                    wifi_driv_prov_config.to_owned(),
                    nvs_partition.to_owned(),
                )
            }),
        )
        .unwrap();

        pc.register_endpoint(
            "prov-scan",
            Box::new(move |ep, data| -> Vec<u8> {
                prov_scan_callback(ep, data, wifi_driv_prov_scan.to_owned())
            }),
        )
        .unwrap();
    }

    fn init_ap(&self) {
        let mut wifi_driv = self.wifi_client.lock().unwrap();
        let apconf = WifiApConfig {
            ssid: self.device_name.clone(),
            ..Default::default()
        };

        wifi_driv.set_ap_config(apconf).unwrap();
    }

    fn print_prov_url(&self) {
        let device_name = &self.device_name;
        let transport = match self.scheme {
            WifiProvScheme::SoftAP => "softap",
            WifiProvScheme::Ble => "ble",
        };

        let data_json = json!({
            "ver":"v1",
            "name": format!("PROV_{device_name}"),
            "transport": transport
        });

        let qr_url = format!(
            "https://espressif.github.io/esp-jumpstart/qrcode.html?data={}",
            data_json
        );

        log::info!(
            "Provisioning started. Visit following url to provision node: {}",
            qr_url
        );
    }
}

fn prov_config_callback(
    _ep: &str,
    data: Vec<u8>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'_>>,
    nvs_partition: NvsPartition,
) -> Vec<u8> {
    let req_proto = WiFiConfigPayload::decode(&*data).unwrap();

    let msg_type = req_proto.msg();
    match msg_type {
        WiFiConfigMsgType::TypeCmdGetStatus => handle_cmd_get_status(wifi_driv),
        WiFiConfigMsgType::TypeCmdSetConfig => {
            handle_cmd_set_config(req_proto.payload.unwrap(), wifi_driv, nvs_partition)
        }
        WiFiConfigMsgType::TypeCmdApplyConfig => handle_cmd_apply_config(),
        _ => unreachable!(),
    }
}

fn prov_scan_callback(
    _ep: &str,
    data: Vec<u8>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'_>>,
) -> Vec<u8> {
    let req_proto = WiFiScanPayload::decode(&*data).unwrap();
    let msg_type = req_proto.msg();

    match msg_type {
        WiFiScanMsgType::TypeCmdScanStart => handle_cmd_scan_start(),
        WiFiScanMsgType::TypeCmdScanStatus => handle_cmd_scan_status(),
        WiFiScanMsgType::TypeCmdScanResult => handle_cmd_scan_result(wifi_driv),
        _ => unreachable!(),
    }
}

fn handle_cmd_set_config(
    req_payload: wi_fi_config_payload::Payload,
    wifi_driv: WrappedInArcMutex<WifiMgr<'_>>,
    nvs_partition: NvsPartition,
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

            let mut nvs = Nvs::new(nvs_partition, "wifi_creds").unwrap();

            nvs.set_bytes("ssid", config.ssid.as_ref()).unwrap();
            nvs.set_bytes("password", config.passphrase.as_ref())
                .unwrap();

            let res_data = WiFiConfigPayload {
                msg: WiFiConfigMsgType::TypeRespSetConfig.into(),
                payload: Some(wi_fi_config_payload::Payload::RespSetConfig(
                    RespSetConfig {
                        status: Status::Success.into(),
                    },
                )),
            };

            res_data.encode_to_vec()
        }
        _ => unreachable!(),
    }
}

fn handle_cmd_apply_config() -> Vec<u8> {
    let resp_msg = WiFiConfigPayload {
        msg: WiFiConfigMsgType::TypeRespApplyConfig.into(),
        payload: Some(wi_fi_config_payload::Payload::RespApplyConfig(
            RespApplyConfig {
                status: Status::Success.into(),
            },
        )),
    };

    resp_msg.encode_to_vec()
}

fn handle_cmd_get_status(wifi_driv: WrappedInArcMutex<WifiMgr<'_>>) -> Vec<u8> {
    let wifi_driv = wifi_driv.lock().unwrap();
    let wifi_client_config = wifi_driv.get_wifi_config().0.unwrap();
    let ip_addr = wifi_driv.get_ip_addr();

    drop(wifi_driv); // no longer needed

    let wifi_state = WifiConnectedState {
        ip4_addr: ip_addr.to_string(),
        auth_mode: components::protocomm::WifiAuthMode::from(wifi_client_config.auth).into(),
        ssid: wifi_client_config.ssid.into(), // to vector
        bssid: vec![],
        channel: 0,
    };

    let resp_msg = WiFiConfigPayload {
        msg: WiFiConfigMsgType::TypeRespGetStatus.into(),
        payload: Some(wi_fi_config_payload::Payload::RespGetStatus(
            RespGetStatus {
                status: Status::Success.into(),
                sta_state: WifiStationState::Connected.into(),
                state: Some(resp_get_status::State::Connected(wifi_state)),
            },
        )),
    };

    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_start() -> Vec<u8> {
    log::info!("Starting wifi scan");
    let resp_msg = WiFiScanPayload {
        msg: WiFiScanMsgType::TypeRespScanStart.into(),
        status: Status::Success.into(),
        payload: Some(wi_fi_scan_payload::Payload::RespScanStart(RespScanStart {})),
    };

    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_status() -> Vec<u8> {
    let resp_msg = WiFiScanPayload {
        msg: WiFiScanMsgType::TypeRespScanStatus.into(),
        status: Status::Success.into(),
        payload: Some(wi_fi_scan_payload::Payload::RespScanStatus(
            RespScanStatus {
                scan_finished: true,
                result_count: 1,
            },
        )),
    };

    resp_msg.encode_to_vec()
}

fn handle_cmd_scan_result(wifi_driv: WrappedInArcMutex<WifiMgr<'_>>) -> Vec<u8> {
    log::info!("Sending scan results");

    let mut resp_msg = WiFiScanPayload {
        msg: WiFiScanMsgType::TypeRespScanResult.into(),
        status: Status::Success.into(),
        ..Default::default()
    };

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
