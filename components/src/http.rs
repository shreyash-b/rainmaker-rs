pub(crate) mod base;
pub use base::*;

mod http_esp;
mod http_linux;

#[cfg(target_os = "espidf")]
pub type HttpServer<'a> = base::HttpServer<esp_idf_svc::http::server::EspHttpServer<'a>>;

#[cfg(target_os = "linux")]
#[allow(private_interfaces)] // HttpServerLinux need not be visible outside
pub type HttpServer<'a> = base::HttpServer<http_linux::HttpServerLinux<'a>>;
