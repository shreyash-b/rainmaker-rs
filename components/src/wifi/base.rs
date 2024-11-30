#[derive(Debug)]
pub struct WifiMgr<T> {
    #[allow(dead_code)]
    pub(crate) client: T,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum WifiAuthMode {
    #[default]
    None,
    WEP,
    WPA,
    WPA2Personal,
    WPAWPA2Personal,
    WPA2Enterprise,
    WPA3Personal,
    WPA2WPA3Personal,
    WAPIPersonal,
}

#[derive(Debug, Default)]
pub struct WifiClientConfig {
    pub ssid: String,
    pub bssid: Vec<u8>,
    pub auth: WifiAuthMode,
    pub password: String,
    pub channel: u8,
}

#[derive(Debug, Default)]
pub struct WifiApConfig {
    pub ssid: String,
    pub password: String,
    pub auth: WifiAuthMode,
}

#[derive(Debug, Default, Clone)]
pub struct WifiApInfo {
    pub ssid: String,
    pub auth: WifiAuthMode,
    pub bssid: Vec<u8>,
    pub channel: u8,
    pub signal_strength: i8,
}
