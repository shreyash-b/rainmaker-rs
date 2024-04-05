use core::fmt;

#[derive(Debug)]
pub struct RMakerError(pub String);

impl fmt::Display for RMakerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.0.clone();
        write!(f, "{}", msg)
    }
}

impl From<components::error::Error> for RMakerError {
    fn from(value: components::error::Error) -> Self {
        let msg = value.to_string();
        Self(msg)
    }
}

#[cfg(target_os = "linux")]
impl From<log::SetLoggerError> for RMakerError {
    fn from(value: log::SetLoggerError) -> Self {
        let msg = value.to_string();
        Self(msg)
    }
}

impl From<std::io::Error> for RMakerError {
    fn from(value: std::io::Error) -> Self {
        let msg = value.to_string();
        Self(msg)
    }
}

#[cfg(target_os = "espidf")]
impl From<esp_idf_svc::sys::EspError> for RMakerError {
    fn from(value: esp_idf_svc::sys::EspError) -> Self {
        let msg = value.to_string();
        let msg = format!("EspError: {}", msg);
        Self(msg)
    }
}

#[cfg(target_os = "espidf")]
impl From<esp_idf_svc::hal::io::EspIOError> for RMakerError {
    fn from(value: esp_idf_svc::hal::io::EspIOError) -> Self {
        value.0.into() // convert EspIoError -> EspError -> Error
    }
}