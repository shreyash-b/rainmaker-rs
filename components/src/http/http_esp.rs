#![cfg(target_os = "espidf")]

use embedded_svc::http::Headers;

use esp_idf_svc::{
    http::{
        server::{EspHttpConnection, EspHttpServer, Request},
        // Headers,
    },
    io::{Read, Write},
};

use crate::{error::Error, http::base::*};

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

impl From<HttpMethod> for esp_idf_svc::http::Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::GET => esp_idf_svc::http::Method::Get,
            HttpMethod::POST => esp_idf_svc::http::Method::Post,
        }
    }

    /* fn from(self) -> esp_idf_svc::http::Method {
    } */
}
// from esp_idf_http_request to custom_request
impl From<&mut Request<&mut EspHttpConnection<'_>>> for HttpRequest {
    fn from(req: &mut Request<&mut EspHttpConnection>) -> Self {
        let buf_len = match req.content_len() {
            Some(l) => l as usize,
            None => 0,
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

impl HttpServer<esp_idf_svc::http::server::EspHttpServer<'static>> {
    pub fn new(config: HttpConfiguration) -> Result<Self, Error> {
        let http_config = esp_idf_svc::http::server::Configuration {
            http_port: config.port,
            ..Default::default()
        };
        Ok(HttpServer(EspHttpServer::new(&http_config)?))
    }

    pub fn add_listener(
        &mut self,
        path: String,
        method: HttpMethod,
        callback: Box<dyn Fn(HttpRequest) -> HttpResponse + Send + 'static>,
    ) {
        self.0
            .fn_handler(
                path.as_str(),
                method.into(),
                move |mut req| -> Result<(), Error> {
                    let user_req = HttpRequest::from(&mut req);
                    let user_response = callback(user_req);
                    req.into_ok_response()?
                        .write_all(&user_response.get_bytes_vectored())?;
                    Ok(())
                },
            )
            .unwrap();
    }
}
