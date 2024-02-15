use core::fmt;

#[derive(Debug)]
pub struct Error(String);

impl std::error::Error for Error{}

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.0.clone();
        write!(f, "{}", msg)
    }
}

#[cfg(target_os="espidf")]
impl From<esp_idf_svc::sys::EspError> for Error{
    fn from(value: esp_idf_svc::sys::EspError) -> Self {
        let msg = value.to_string();
        let msg = format!("EspError: {}", msg);
        Self(msg)
    }
}