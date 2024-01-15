#![cfg(target_os = "espidf")]

use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::prelude::Peripherals,
    wifi::{
        AccessPointConfiguration, AuthMethod, BlockingWifi, ClientConfiguration, Configuration,
        EspWifi,
    },
};

pub struct WifiMgr<'a> {
    wifi: BlockingWifi<EspWifi<'a>>,
}

impl WifiMgr<'_> {
    pub fn new() -> Self {
        let sysloop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();

        let wifi_client = BlockingWifi::wrap(
            EspWifi::new(
                Peripherals::take().unwrap().modem,
                sysloop.clone(),
                Some(nvs),
            )
            .unwrap(),
            sysloop,
        )
        .unwrap();

        Self { wifi: wifi_client }
    }

    pub fn set_softap(&mut self) {
        log::info!("setting wifi config to softap");
        let apconfig = AccessPointConfiguration::default();

        self.wifi
            .set_configuration(&Configuration::AccessPoint(apconfig))
            .unwrap()
    }

    pub fn set_client(&mut self, ssid: &str, key: &str) {
        let mut clientconfig = ClientConfiguration::default();
        clientconfig.ssid = ssid.into();

        if key.len() > 0 {
            log::info!("setting wifi key");
            clientconfig.password = key.into();
            clientconfig.auth_method = AuthMethod::WPA2Personal;
        }

        let old_config = self.wifi.get_configuration().unwrap();

        // self.wifi.stop().unwrap();

        log::info!(
            "setting wifi config as client: {}, {}",
            clientconfig.ssid,
            clientconfig.password
        );
        self.wifi
            .set_configuration(&Configuration::Mixed(
                clientconfig,
                old_config.as_ap_conf_ref().unwrap().to_owned(),
            ))
            .unwrap();
    }

    pub fn start(&mut self) {
        log::info!("trying to start wifi");
        self.wifi.start().unwrap();
        self.wifi.connect().unwrap();

        while !self.wifi.is_connected().unwrap() {
            log::info!("waiting for wifi to connect");
            Delay::new_default().delay_ms(100);
        }

        log::info!("wifi start done")
    }
}
