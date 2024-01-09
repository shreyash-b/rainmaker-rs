use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}};


pub enum HttpMethod {
    GET,
    POST,
    OTHER
}

pub struct HttpRequest {
    pub method: HttpMethod,
    
}

pub struct HttpResponse<'a>{
    data: &'a [u8]
}

impl<'a> HttpResponse<'a> {
    pub fn from_bytes(inp: &'a [u8]) -> Self{
        HttpResponse{
            data: inp
        }
    }

    pub fn get_bytes(&self) -> &[u8]{
        self.data
    }
}

pub struct HttpConfiguration {
    pub port: u16,
    pub addr: IpAddr
}

impl Default for HttpConfiguration {
    fn default() -> Self {
        HttpConfiguration{
            // to avoid running prog as sudo during dev
            port: 8080,
            addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
        }
    }
}

pub struct HttpServer<'a, T, U>{
    pub(crate) server: T,
    #[allow(dead_code)]
    pub(crate) listeners: Option<HashMap<&'a str, U>>
}
