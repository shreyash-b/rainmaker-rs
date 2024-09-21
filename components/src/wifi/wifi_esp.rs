#![cfg(target_os = "espidf")]

use std::str::FromStr;

use crate::error::Error;
use crate::wifi::base::*;

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::Delay, modem::Modem},
    ipv4::{self, Ipv4Addr, Mask, Subnet},
    netif::{EspNetif, NetifConfiguration, NetifStack},
    wifi::{
        AccessPointConfiguration, AccessPointInfo, AuthMethod, BlockingWifi, ClientConfiguration,
        Configuration, EspWifi, WifiDriver,
    },
};

impl From<WifiAuthMode> for AuthMethod {
    fn from(value: WifiAuthMode) -> Self {
        match value {
            WifiAuthMode::None => Self::None,
            WifiAuthMode::WEP => Self::WEP,
            WifiAuthMode::WPA => Self::WPA,
            WifiAuthMode::WPA2Personal => Self::WPA2Personal,
            WifiAuthMode::WPAWPA2Personal => Self::WPAWPA2Personal,
            WifiAuthMode::WPA2Enterprise => Self::WPA2Enterprise,
            WifiAuthMode::WPA3Personal => Self::WPA3Personal,
            WifiAuthMode::WPA2WPA3Personal => Self::WPA2WPA3Personal,
            WifiAuthMode::WAPIPersonal => Self::WAPIPersonal,
        }
    }
}

// https://i.pinimg.com/736x/3f/cb/2b/3fcb2b34d2d0d24fc888be5d6f6a4e84.jpg

impl From<AuthMethod> for WifiAuthMode {
    fn from(value: AuthMethod) -> Self {
        match value {
            AuthMethod::None => Self::None,
            AuthMethod::WEP => Self::WEP,
            AuthMethod::WPA => Self::WPA,
            AuthMethod::WPA2Personal => Self::WPA2Personal,
            AuthMethod::WPAWPA2Personal => Self::WPAWPA2Personal,
            AuthMethod::WPA2Enterprise => Self::WPA2Enterprise,
            AuthMethod::WPA3Personal => Self::WPA3Personal,
            AuthMethod::WPA2WPA3Personal => Self::WPA2WPA3Personal,
            AuthMethod::WAPIPersonal => Self::WAPIPersonal,
        }
    }
}

impl From<AccessPointInfo> for WifiApInfo {
    fn from(value: AccessPointInfo) -> Self {
        Self {
            ssid: value.ssid.as_str().into(),
            auth: value.auth_method.unwrap().into(),
            bssid: value.bssid.into(),
            channel: value.channel,
            signal_strength: value.signal_strength,
        }
    }
}

impl From<WifiApConfig> for AccessPointConfiguration {
    fn from(value: WifiApConfig) -> Self {
        AccessPointConfiguration {
            ssid: heapless::String::from_str(&value.ssid).unwrap(),
            password: heapless::String::from_str(&value.password).unwrap(),
            auth_method: value.auth.into(),
            ..Default::default()
        }
    }
}

impl From<AccessPointConfiguration> for WifiApConfig {
    fn from(value: AccessPointConfiguration) -> Self {
        Self {
            ssid: value.ssid.as_str().into(),
            password: value.password.as_str().into(),
            auth: value.auth_method.into(),
        }
    }
}

impl From<WifiClientConfig> for ClientConfiguration {
    fn from(value: WifiClientConfig) -> Self {
        let bssid = match value.bssid.try_into() {
            Ok(v) => Some(v),
            Err(_) => None,
        };
        Self {
            ssid: heapless::String::from_str(&value.ssid).unwrap(),
            bssid,
            auth_method: value.auth.into(),
            password: heapless::String::from_str(&value.password).unwrap(),
            channel: Some(value.channel),
            ..Default::default()
        }
    }
}

impl From<ClientConfiguration> for WifiClientConfig {
    fn from(value: ClientConfiguration) -> Self {
        Self {
            ssid: value.ssid.as_str().into(),
            bssid: value.bssid.unwrap_or([0; 6]).to_vec(),
            auth: value.auth_method.into(),
            password: value.password.as_str().into(),
            channel: value.channel.unwrap(),
        }
    }
}

impl WifiMgr<BlockingWifi<EspWifi<'_>>> {
    pub fn new() -> Result<Self, Error> {
        let sysloop = EspSystemEventLoop::take()?;
        let modem = unsafe { Modem::new() };

        // netif configuration defaults to 192.168.71.1 however wifi provisioning using softap requires 192.168.4.1
        // so use custom router configuration
        let mut netif_router_config = NetifConfiguration::wifi_default_router();
        netif_router_config.ip_configuration =
            ipv4::Configuration::Router(ipv4::RouterConfiguration {
                subnet: Subnet {
                    gateway: Ipv4Addr::new(192, 168, 4, 1),
                    mask: Mask(24),
                },
                ..Default::default()
            });

        let inner_client = EspWifi::wrap_all(
            WifiDriver::new(modem, sysloop.clone(), None)?,
            EspNetif::new(NetifStack::Sta)?,
            EspNetif::new_with_conf(&netif_router_config)?,
        )?;

        let mut wifi_client = BlockingWifi::wrap(inner_client, sysloop)?;

        // configuration defaults to sta + softap, and we don't want that
        wifi_client.set_configuration(&Configuration::None)?;

        Ok(Self {
            client: wifi_client,
        })
    }

    pub fn set_ap_config(&mut self, config: WifiApConfig) -> Result<(), Error> {
        let apconfig = AccessPointConfiguration::from(config);
        let wifi_config = match self.client.get_configuration()?.as_client_conf_ref() {
            Some(config) => Configuration::Mixed(config.to_owned(), apconfig),
            None => {
                // for some reason esp_idf_svc sets 192.168.4.1 as the default gateway
                Configuration::AccessPoint(apconfig)
            }
        };

        self.client.set_configuration(&wifi_config)?;
        Ok(())
    }

    pub fn set_client_config(&mut self, config: WifiClientConfig) -> Result<(), Error> {
        let staconfig = ClientConfiguration::from(config);
        let wifi_config = match self.client.get_configuration()?.as_ap_conf_ref() {
            Some(config) => Configuration::Mixed(staconfig, config.to_owned()),
            None => Configuration::Client(staconfig),
        };

        self.client.set_configuration(&wifi_config)?;

        self.check_reconnect_wifi()?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.client.start()?;
        Ok(())
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        match self.client.get_configuration()? {
            Configuration::None => {
                log::error!("cannot connect wifi: no config set");
                return Ok(());
            }
            Configuration::AccessPoint(_) => {
                log::error!("cannot connect wifi: wifi in ap mode");
                return Ok(());
            }
            config => config,
        };

        self.client.connect()?;

        Ok(())
    }

    pub fn assured_connect(&mut self) {
        while self.connect().is_err() {
            log::warn!("Unable to connect to wifi. Retrying");
            Delay::new_default().delay_ms(1000);
        }

        self.client.wait_netif_up().unwrap();
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected().unwrap()
    }

    pub fn get_wifi_config(&self) -> (Option<WifiClientConfig>, Option<WifiApConfig>) {
        match self.client.get_configuration().unwrap() {
            Configuration::None => (None, None),
            Configuration::Client(client_config) => (Some(client_config.into()), None),
            Configuration::AccessPoint(ap_config) => (None, Some(ap_config.into())),
            Configuration::Mixed(client_config, ap_config) => {
                (Some(client_config.into()), Some(ap_config.into()))
            }
        }
    }

    pub fn get_ip_addr(&self) -> std::net::Ipv4Addr {
        // TODO

        std::net::Ipv4Addr::new(127, 0, 0, 1) // dummy
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        self.client.disconnect()?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.client.stop()?;
        Ok(())
    }

    pub fn scan(&mut self) -> Result<Vec<WifiApInfo>, Error> {
        let scan_networks = self.client.scan()?;
        let mut ret_networks = Vec::<WifiApInfo>::new();

        for netork in scan_networks.iter() {
            ret_networks.push(netork.to_owned().into())
        }

        Ok(ret_networks)
    }

    #[allow(dead_code)]
    fn check_restart_wifi(&mut self) -> Result<(), Error> {
        let wifi_connected = self.client.is_connected()?;

        if self.client.is_started()? {
            log::warn!("restarting wifi");
            self.stop()?;
            self.start()?;
            if wifi_connected {
                self.connect()?;
            }
        };

        Ok(())
    }

    fn check_reconnect_wifi(&mut self) -> Result<(), Error> {
        if self.client.is_connected()? {
            log::warn!("reconnecting wifi");
            self.disconnect()?;
            esp_idf_svc::hal::delay::Delay::new_default().delay_ms(2000);
            self.connect()?;
        }

        Ok(())
    }
}
