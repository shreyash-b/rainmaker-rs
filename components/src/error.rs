use core::fmt;

#[derive(Debug)]
pub struct Error(pub(crate) String);

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.0.clone();
        write!(f, "{}", msg)
    }
}

#[cfg(target_os = "espidf")]
impl From<esp_idf_svc::sys::EspError> for Error {
    fn from(value: esp_idf_svc::sys::EspError) -> Self {
        let msg = value.to_string();
        let msg = format!("EspError: {}", msg);
        Self(msg)
    }
}

#[cfg(target_os = "espidf")]
impl From<esp_idf_svc::hal::io::EspIOError> for Error {
    fn from(value: esp_idf_svc::hal::io::EspIOError) -> Self {
        value.0.into() // convert EspIoError -> EspError -> Error
    }
}

#[cfg(target_os = "linux")]
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        let msg = value.to_string();
        let msg = format!("IoError: {}", msg);

        Self(msg)
    }
}

#[cfg(target_os = "linux")]
impl From<pickledb::error::Error> for Error {
    fn from(value: pickledb::error::Error) -> Self {
        let msg = format!("PickleDb Error: {}", value);

        Self(msg)
    }
}

#[cfg(target_os = "linux")]
impl From<bluer::Error> for Error {
    fn from(value: bluer::Error) -> Self {
        let msg = format!("Bluer Error: {}", value.message);

        Self(msg)
    }
}
