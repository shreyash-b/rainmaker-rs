use crate::wifi::base::*;

impl WifiMgr<()> {
    pub fn new() -> Self {
        Self { client: () }
    }

    pub fn set_softap_config(&mut self, _config: WifiConfig) {}

    pub fn set_client_config(&mut self, _config: WifiConfig) {}

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}

    pub fn connect(&mut self) {}
}
