#![cfg(target_os = "espidf")]

use crate::http_client::base::*;

const KEEP_ALIVE_TIMEOUT: u64 = 50000;
const HTTP_TX_BUFFER_SIZE: usize = 2048;

impl From<esp_idf_svc::http::Method> for HttpMethod {
    fn from(value: esp_idf_svc::http::Method) -> Self {
        match value {
            esp_idf_svc::http::Method::Get => HttpMethod::GET,
            esp_idf_svc::http::Method::Post => HttpMethod::POST,
            _ => unreachable!(),
        }
    }
}

impl Into<esp_idf_svc::http::Method> for HttpMethod {
    fn into(self) -> esp_idf_svc::http::Method {
        match self {
            Self::GET => esp_idf_svc::http::Method::Get,
            Self::POST => esp_idf_svc::http::Method::Post,
        }
    }
}

impl HttpClient<embedded_svc::http::client::Client<esp_idf_svc::http::client::EspHttpConnection>> {
    pub fn new() -> Result<Self, esp_idf_hal::sys::EspError> {
        let mut configuration = esp_idf_svc::http::client::Configuration::default();
        configuration.timeout = Some(std::time::Duration::from_millis(KEEP_ALIVE_TIMEOUT));
        configuration.buffer_size_tx = Some(HTTP_TX_BUFFER_SIZE);

        let httpconnection = esp_idf_svc::http::client::EspHttpConnection::new(&configuration)?;

        let httpclient = embedded_svc::http::client::Client::wrap(httpconnection);

        Ok(Self { client: httpclient })
    }

}
