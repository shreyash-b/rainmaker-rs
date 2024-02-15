use crate::wifi::base::*;

impl WifiMgr<()> {
    pub fn new() -> Self {
        Self { client: () }
    }

    pub fn set_ap_config(&mut self, _config: WifiApConfig) {}

    pub fn set_client_config(&mut self, _config: WifiClientConfig) {}

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}

    pub fn connect(&mut self) {}

    pub fn scan(&mut self) -> Vec<WifiApInfo> {
        vec![WifiApInfo{
            ssid: "DUMMY_NETWORK_LINUX".to_string(),
            auth: WifiAuthMode::None,
            ..Default::default()
        }]
    }

    pub fn get_wifi_config(&self) -> (Option<WifiClientConfig>, Option<WifiApConfig>) {
        (Some(WifiClientConfig::default()), Some(WifiApConfig::default()))
    }
}
