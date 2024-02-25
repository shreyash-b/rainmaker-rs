pub use prost::Message;
include!(concat!(env!("OUT_DIR"), "/session.rs"));

impl From<crate::wifi::WifiAuthMode> for WifiAuthMode {
    fn from(value: crate::wifi::WifiAuthMode) -> Self {
        match value {
            crate::wifi::WifiAuthMode::None => Self::Open,
            crate::wifi::WifiAuthMode::WEP => Self::Wep,
            crate::wifi::WifiAuthMode::WPA => Self::WpaPsk,
            crate::wifi::WifiAuthMode::WPA2Personal => Self::Wpa2Psk,
            crate::wifi::WifiAuthMode::WPAWPA2Personal => Self::WpaWpa2Psk,
            crate::wifi::WifiAuthMode::WPA2Enterprise => Self::Wpa2Enterprise,
            crate::wifi::WifiAuthMode::WPA3Personal => Self::Wpa3Psk,
            crate::wifi::WifiAuthMode::WPA2WPA3Personal => Self::Wpa2Wpa3Psk,
            crate::wifi::WifiAuthMode::WAPIPersonal => Self::WpaPsk,
        }
    }
}
