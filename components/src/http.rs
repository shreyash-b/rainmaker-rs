pub mod base;
pub use base::*;

#[cfg(target_os="linux")]
mod http_linux;

#[cfg(target_os="espidf")]
mod http_esp;

// todo: concurrency on linux
