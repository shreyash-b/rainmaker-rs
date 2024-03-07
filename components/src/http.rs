pub(crate) mod base;
pub use base::*;

#[cfg(target_os = "espidf")]
mod http_esp;

// #[cfg(target_os="espidf")]
// pub use http_esp::*;

#[cfg(target_os = "espidf")]
pub type HttpServer<'a> = base::HttpServer<'a, esp_idf_svc::http::server::EspHttpServer<'a>>;

#[cfg(target_os = "linux")]
mod http_linux;

// #[cfg(target_os="linux")]
// pub use http_linux::*;

#[cfg(target_os = "linux")]
pub type HttpServer<'a> = base::HttpServer<'a, tiny_http::Server>;

// todo: concurrency on linux
