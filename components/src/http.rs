pub(crate) mod base;
pub use base::*;

mod http_esp;
mod http_linux;

#[cfg(target_os = "espidf")]
pub type HttpServer = base::HttpServer<esp_idf_svc::http::server::EspHttpServer<'static>>;

#[cfg(target_os = "linux")]
pub type HttpServer = base::HttpServer<http_linux::HttpServerLinux>;
