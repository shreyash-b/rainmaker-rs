pub struct WifiMgr<T> {
    #[allow(dead_code)]
    pub(crate) client: T,
}

#[derive(Default)]
pub enum WifiAuthMode {
    #[default]
    None,
    Wpa2Personal,
}

#[derive(Default)]
pub struct WifiConfig {
    pub ssid: String,
    pub key: String,
    pub auth: WifiAuthMode,
}

impl From<&WifiAuthMode> for crate::protocomm::WifiAuthMode{
    fn from(value: &WifiAuthMode) -> Self {
        match value{
            WifiAuthMode::None => Self::Open,
            WifiAuthMode::Wpa2Personal => Self::Wpa2Psk,
        }
    }
}
