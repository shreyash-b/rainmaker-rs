mod base;

pub use base::*;

#[cfg(target_os="espidf")]
mod persistent_esp;

#[cfg(target_os="linux")]
mod persistent_linux;