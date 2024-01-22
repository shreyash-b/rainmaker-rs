mod base;
pub use base::*;

#[cfg(target_os = "espidf")]
mod wifi_esp;

#[cfg(target_os = "espidf")]
pub use wifi_esp::*;

#[cfg(target_os = "espidf")]
pub type WifiMgr<'a> = base::WifiMgr<esp_idf_svc::wifi::BlockingWifi<esp_idf_svc::wifi::EspWifi<'a>>>;

#[cfg(target_os = "linux")]
mod wifi_linux;

#[cfg(target_os = "linux")]
pub use wifi_linux::*;

#[cfg(target_os="linux")]
pub type WifiMgr<'a> = base::WifiMgr<()>;