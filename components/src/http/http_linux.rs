#![cfg(target_os="linux")]

use std::net::SocketAddr;
use std::collections::HashMap;

use crate::http::base::*;

impl From<&tiny_http::Method> for HttpMethod{
    fn from(inp: &tiny_http::Method) -> Self {
        match inp {
            tiny_http::Method::Get => HttpMethod::GET,
            tiny_http::Method::Post => HttpMethod::POST,
            _ => HttpMethod::OTHER
        }
    }
}

impl From<&mut tiny_http::Request> for HttpRequest {
    fn from(req: &mut tiny_http::Request) -> Self {

        let req_len = req.body_length();
        let mut buf: Vec<u8>;
        match req_len {
            Some(len) => {
                buf = Vec::<u8>::with_capacity(len);
                let reader = req.as_reader();
                reader.read_to_end(&mut buf).unwrap();
            },

            None => buf = Vec::default(),
        }

        Self{
            method: req.method().into(),
            data: buf
        }
    }

}

impl<'a, U> HttpServer<'a, tiny_http::Server, U>
where
    U: Fn(HttpRequest) -> HttpResponse<'a>
{


    pub fn new(config: &HttpConfiguration) -> anyhow::Result<Self>{
        Ok(HttpServer{
            server: tiny_http::Server::http(SocketAddr::new(config.addr, config.port)).unwrap(),
            listeners: Some(HashMap::new())
        })
    }

    pub fn add_listener(&mut self, path: &'a str, callback: U){
        if let Some(listeners) = self.listeners.as_mut() {
            listeners.insert(path, callback);
        }

    }


    pub fn listen(&self) -> anyhow::Result<()> {
         loop {
            log::info!("http server is listening");
            let mut req = self.server.recv().unwrap();
            let http_request = HttpRequest::from(&mut req);
            let req_callback = self.listeners.as_ref().unwrap().get(req.url());
            let response = match req_callback {
                Some(c) => c(http_request),
                None => HttpResponse::from_bytes("not found".as_bytes())
            };

            req.respond(tiny_http::Response::from_data(response.get_bytes())).unwrap()
        }
    }
}

