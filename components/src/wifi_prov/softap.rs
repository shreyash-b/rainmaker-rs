use crate::{
    protocomm::{ProtocommHttpd, ProtocommSecurity},
    utils::WrappedInArcMutex,
    wifi::{WifiApConfig, WifiMgr},
};

use super::WiFiProvTransportTrait;

#[derive(Debug, Default, Clone)]
pub struct WiFiProvSoftApConfig {
    pub service_name: String,
    pub service_key: Option<String>,
}

pub struct WiFiProvTransportSoftAp {
    pc: ProtocommHttpd,
    wifi: WrappedInArcMutex<WifiMgr<'static>>,
    service_name: String,
    service_key: Option<String>,
}

impl WiFiProvTransportSoftAp {
    pub fn new(
        config: WiFiProvSoftApConfig,
        sec: ProtocommSecurity,
        wifi: WrappedInArcMutex<WifiMgr<'static>>,
    ) -> Self {
        let pc = ProtocommHttpd::new(
            crate::http::HttpConfiguration {
                port: 80,
                ..Default::default()
            },
            sec,
        );
        Self {
            pc,
            service_name: config.service_name,
            service_key: config.service_key,
            wifi,
        }
    }
}

impl WiFiProvTransportTrait for WiFiProvTransportSoftAp {
    fn start(&mut self) -> Result<(), crate::error::Error> {
        let mut wifi_ap_config = WifiApConfig {
            ssid: self.service_name.clone(),
            ..Default::default()
        };

        let key = &self.service_key;
        if key.is_some() {
            wifi_ap_config.password = key.as_ref().unwrap().clone();
        }

        let mut wifi = self.wifi.lock().unwrap();
        wifi.set_ap_config(wifi_ap_config).unwrap();
        Ok(())
    }

    fn add_endpoint(&mut self, ep_name: &str, cb: crate::protocomm::ProtocommCallbackType) {
        self.pc.register_endpoint(ep_name, cb);
    }

    fn set_version_info(&mut self, ep_name: &str, version_info: String) {
        self.pc.set_version_info(ep_name, version_info);
    }

    fn set_security_ep(&mut self, ep_name: &str) {
        self.pc.set_security_endpoint(ep_name);
    }
}
