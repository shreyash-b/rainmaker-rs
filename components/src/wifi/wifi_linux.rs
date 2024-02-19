use crate::wifi::base::*;
use crate::error::Error;

impl WifiMgr<()> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self { client: () })
    }

    pub fn set_ap_config(&mut self, _config: WifiApConfig) -> Result<(), Error>{
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

    pub fn assured_connect(&mut self){

    }

    pub fn scan(&mut self) -> Result<Vec<WifiApInfo>, Error> {
        Ok(vec![WifiApInfo{
            ssid: "DUMMY_NETWORK_LINUX".to_string(),
            auth: WifiAuthMode::None,
            ..Default::default()
        }])
    }

    pub fn get_wifi_config(&self) -> (Option<WifiClientConfig>, Option<WifiApConfig>) {
        (Some(WifiClientConfig::default()), Some(WifiApConfig::default()))
    }
}
