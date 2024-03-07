use crate::error::Error;
use crate::wifi::base::*;

impl WifiMgr<()> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self { client: () })
    }

    pub fn set_ap_config(&mut self, _config: WifiApConfig) -> Result<(), Error> {
        Ok(())
    }

    pub fn set_client_config(&mut self, _config: WifiClientConfig) -> Result<(), Error> {
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn assured_connect(&mut self) {
        self.connect().unwrap()
    }

    pub fn is_connected(&self) -> bool {
        true
    }

    pub fn scan(&mut self) -> Result<Vec<WifiApInfo>, Error> {
        Ok(vec![WifiApInfo {
            ssid: "DUMMY_NETWORK_LINUX".to_string(),
            auth: WifiAuthMode::None,
            ..Default::default()
        }])
    }

    pub fn get_ip_addr(&self) -> std::net::Ipv4Addr {
        // TODO

        std::net::Ipv4Addr::new(127, 0, 0, 1) // dummy
    }

    pub fn get_wifi_config(&self) -> (Option<WifiClientConfig>, Option<WifiApConfig>) {
        (
            Some(WifiClientConfig::default()),
            Some(WifiApConfig::default()),
        )
    }
}
