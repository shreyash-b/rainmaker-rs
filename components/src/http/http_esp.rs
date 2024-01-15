#![cfg(target_os="espidf")]

use embedded_svc::{http::Headers, io::Read};
use esp_idf_svc::{
    http::server::{EspHttpConnection, Request},
    io::Write,
};

use crate::http::base::*;

impl From<esp_idf_svc::http::Method> for HttpMethod {
    fn from(value: esp_idf_svc::http::Method) -> Self {
        use esp_idf_svc::http::Method;
        match value {
            Method::Get => HttpMethod::GET,
            Method::Post => HttpMethod::POST,
            _ => unreachable!(),
        }
    }
}

impl Into<esp_idf_svc::http::Method> for HttpMethod{
    fn into(self) -> esp_idf_svc::http::Method {
        use esp_idf_svc::http::Method;
        match self{
            Self::GET => Method::Get,
            Self::POST => Method::Post, 
        }

    }
}

impl From<&mut Request<&mut EspHttpConnection<'_>>> for HttpRequest {
    fn from(req: &mut Request<&mut EspHttpConnection>) -> Self {
        let buf_len = match req.content_len() {
            Some(l) => l as usize,
            None => 0
        };
        let mut buf = vec![0; buf_len];
        req.read_exact(&mut buf).unwrap();

        Self {
            method: req.method().into(),
            url: req.uri().to_owned(),
            data: buf,

        }
    }
}

impl<'a> HttpServer<'a, esp_idf_svc::http::server::EspHttpServer<'a>, Box<dyn Fn(HttpRequest) -> HttpResponse + Send>> {
    pub fn new(config: &HttpConfiguration) -> anyhow::Result<Self> {
        let mut http_config = esp_idf_svc::http::server::Configuration::default();
        http_config.http_port = config.port;
        Ok(HttpServer {
            server: esp_idf_svc::http::server::EspHttpServer::new(&http_config).unwrap(),
            listeners: None,
        })
    }

    pub fn add_listener(&mut self, path: &'a str, method: HttpMethod,  callback: Box<dyn Fn(HttpRequest) -> HttpResponse + Send>) {
        self.server
            .fn_handler(path, method.into(), move |mut req| {
                let user_req = HttpRequest::from(&mut req);
                let user_response = callback(user_req);
                req.into_ok_response()
                    .unwrap()
                    .write_all(&user_response.get_bytes_vectored())
                    .unwrap();
                Ok(())
            })
            .unwrap();
    }

    pub fn listen(&self) -> anyhow::Result<()> {
        log::info!("http server is listening");
        loop {
            esp_idf_svc::hal::delay::Delay::new_default().delay_ms(1000)
        }
    }
}
