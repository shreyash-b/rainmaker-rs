use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub(crate) method: HttpMethod,
    // todo: convert to stream
    pub(crate) data: Vec<u8>,
    pub(crate) url: String,
}

impl HttpRequest {
    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn data(&mut self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    data: Vec<u8>,
}

impl HttpResponse {
    pub fn from_bytes<D>(inp: D) -> Self
    where
        D: Into<Vec<u8>>,
    {
        HttpResponse { data: inp.into() }
    }

    pub fn get_bytes_vectored(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(Debug)]
pub struct HttpConfiguration {
    pub port: u16,
    pub addr: IpAddr,
}

impl Default for HttpConfiguration {
    fn default() -> Self {
        HttpConfiguration {
            #[cfg(target_os = "espidf")]
            port: 80,
            // to avoid running prog as sudo during dev
            #[cfg(target_os = "linux")]
            port: 8080,
            addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
}

pub struct HttpServer<T>(pub(crate) T);
