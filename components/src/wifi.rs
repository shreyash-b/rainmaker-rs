mod base;
pub use base::*;

mod wifi_esp;
mod wifi_linux;

#[cfg(target_os = "espidf")]
pub type WifiMgr<'a> =
    base::WifiMgr<esp_idf_svc::wifi::BlockingWifi<esp_idf_svc::wifi::EspWifi<'a>>>;

#[cfg(target_os = "linux")]
pub type WifiMgr<'a> = base::WifiMgr<()>;
