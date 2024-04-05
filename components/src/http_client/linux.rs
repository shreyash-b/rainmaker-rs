#![cfg(target_os = "linux")]

use reqwest;

use super::{HttpClient, HttpMethod};

impl From<reqwest::Method> for HttpMethod {
    fn from(value: reqwest::Method) -> Self {
        match value {
            reqwest::Method::GET => HttpMethod::GET,
            reqwest::Method::POST => HttpMethod::POST,
            _ => unreachable!(),
        }
    }
}

impl HttpClient<reqwest::Client> {
    pub fn new() -> Result<Self, reqwest::Error> {
        let headers = reqwest::header::HeaderMap::new();
        let httpclient = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self { client: httpclient })
    }
}
