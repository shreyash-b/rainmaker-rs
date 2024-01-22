#![cfg(target_os = "espidf")]

use crate::wifi::base::*;

use embedded_svc::wifi::AccessPointInfo;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::prelude::Peripherals,
    wifi::{
        AccessPointConfiguration, AuthMethod, BlockingWifi, ClientConfiguration, Configuration,
        EspWifi,
    },
};

impl From<WifiAuthMode> for AuthMethod {
    fn from(value: WifiAuthMode) -> Self {
        match value {
            WifiAuthMode::None => AuthMethod::None,
            WifiAuthMode::Wpa2Personal => AuthMethod::WPA2Personal,
        }
    }
}

// https://i.pinimg.com/736x/3f/cb/2b/3fcb2b34d2d0d24fc888be5d6f6a4e84.jpg

impl From<AuthMethod> for WifiAuthMode {
    fn from(value: AuthMethod) -> Self {
        match value {
            AuthMethod::None => Self::None,
            AuthMethod::WEP => todo!(),
            AuthMethod::WPA => todo!(),
            AuthMethod::WPA2Personal => Self::Wpa2Personal,
            AuthMethod::WPAWPA2Personal => todo!(),
            AuthMethod::WPA2Enterprise => todo!(),
            AuthMethod::WPA3Personal => todo!(),
            AuthMethod::WPA2WPA3Personal => todo!(),
            AuthMethod::WAPIPersonal => todo!(),
        }
    }
}

impl From<WifiConfig> for AccessPointConfiguration {
    fn from(value: WifiConfig) -> Self {
        let mut config = AccessPointConfiguration::default();
        config.ssid = value.ssid.as_str().into();
        config.password = value.key.as_str().into();
        config.auth_method = value.auth.into();

        config
    }
}

impl From<AccessPointInfo> for WifiConfig{
    fn from(value: AccessPointInfo) -> Self {
        return Self{
            ssid: value.ssid.as_str().into(),
            auth: value.auth_method.into(),
            ..Default::default()
        }
    }
}

impl From<WifiConfig> for ClientConfiguration {
    fn from(value: WifiConfig) -> Self {
        let mut config = ClientConfiguration::default();
        config.ssid = value.ssid.as_str().into();
        config.password = value.key.as_str().into();
        config.auth_method = value.auth.into();

        config
    }
}

impl WifiMgr<BlockingWifi<EspWifi<'_>>> {
    pub fn new() -> Self {
        // TODO: find alternative for linux and take sysloop as argument
        let sysloop = EspSystemEventLoop::take().unwrap();

        let wifi_client = BlockingWifi::wrap(
            EspWifi::new(
                Peripherals::take().unwrap().modem,
                sysloop.clone(),
                None,
            )
            .unwrap(),
            sysloop,
        )
        .unwrap();    

        Self {
            client: wifi_client,
        }
    }

    pub fn set_softap_config(&mut self, config: WifiConfig) {
        let apconfig = AccessPointConfiguration::from(config);
        let wifi_config: Configuration;

        wifi_config = match self
            .client
            .get_configuration()
            .unwrap()
            .as_client_conf_ref()
        {
            Some(config) => Configuration::Mixed(config.to_owned(), apconfig),
            None => Configuration::AccessPoint(apconfig),
        };

        self.client.set_configuration(&wifi_config).unwrap();
    }

    pub fn set_client_config(&mut self, config: WifiConfig) {
        let staconfig = ClientConfiguration::from(config);
        let wifi_config: Configuration;

        wifi_config = match self.client.get_configuration().unwrap().as_ap_conf_ref() {
            Some(config) => Configuration::Mixed(staconfig, config.to_owned()),
            None => Configuration::Client(staconfig),
        };

        self.client.set_configuration(&wifi_config).unwrap();

        self.check_reconnect_wifi();
    }

    pub fn start(&mut self) {
        self.client.start().unwrap();
    }

    pub fn connect(&mut self) {
        let wifi_config = match self.client.get_configuration().unwrap() {
            Configuration::None => {
                log::error!("cannot connect wifi: no config set");
                return;
            }
            Configuration::AccessPoint(_) => {
                log::error!("cannot connect wifi: wifi in ap mode");
                return;
            }
            config => config,
        };

        let ssid = &wifi_config.as_client_conf_ref().unwrap().ssid;
        self.client.connect().unwrap();

        while !self.client.is_connected().unwrap() {
            log::info!("waiting for ssid={}", ssid);
            esp_idf_svc::hal::delay::Delay::new_default().delay_ms(100);
        }
    }

    pub fn disconnect(&mut self){
        self.client.disconnect().unwrap();
    }

    pub fn stop(&mut self) {
        self.client.stop().unwrap()
    }

    pub fn scan(&mut self) -> Vec<WifiConfig> {
        let scan_networks = self.client.scan().unwrap();
        let mut ret_networks = Vec::<WifiConfig>::new();
        
        for netork in scan_networks.iter() {
            ret_networks.push(netork.to_owned().into())
        };

        ret_networks
    }

    fn check_restart_wifi(&mut self) {
        let wifi_connected = self.client.is_connected().unwrap();

        if self.client.is_started().unwrap() {
            log::warn!("restarting wifi");
            self.stop();
            self.start();
            if wifi_connected {
                self.connect();
            }
        }
    }

    fn check_reconnect_wifi(&mut self) {
        if self.client.is_connected().unwrap() {
            log::warn!("reconnecting wifi");
            self.disconnect();
            esp_idf_svc::hal::delay::Delay::new_default().delay_ms(2000);
            self.connect()
        }
    }
}
