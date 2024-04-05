pub(crate) mod base;
pub use base::*;

#[cfg(target_os = "espidf")]
mod esp;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "espidf")]
pub type HttpClient = base::HttpClient<embedded_svc::http::client::Client<esp_idf_svc::http::client::EspHttpConnection>>;