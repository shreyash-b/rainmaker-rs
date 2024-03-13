#![cfg(target_os = "linux")]

use std::collections::HashMap;
use std::net::SocketAddr;

use crate::http::base::*;

impl From<&tiny_http::Method> for HttpMethod {
    fn from(inp: &tiny_http::Method) -> Self {
        match inp {
            tiny_http::Method::Get => HttpMethod::GET,
            tiny_http::Method::Post => HttpMethod::POST,
            _ => unreachable!(),
        }
    }
}

impl From<&mut tiny_http::Request> for HttpRequest {
    fn from(req: &mut tiny_http::Request) -> Self {
        let buf_len = match req.body_length() {
            Some(l) => l,
            None => 0,
        };
        let mut buf = vec![0; buf_len];
        req.as_reader().read_exact(&mut buf).unwrap();

        Self {
            method: req.method().into(),
            url: req.url().to_string(),
            data: buf,
        }
    }
}

impl<'a> HttpServer<tiny_http::Server> {
    pub fn new(config: &HttpConfiguration) -> anyhow::Result<Self> {
        Ok(HttpServer {
            server: tiny_http::Server::http(SocketAddr::new(config.addr, config.port)).unwrap(),
            listeners: Some(HashMap::new()),
        })
    }

    pub fn add_listener(
        &mut self,
        path: String,
        method: HttpMethod,
        callback: Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>,
    ) {
        // if inner hashmap does not exist for a path, create it
        let paths_hmap = self.listeners.as_mut().unwrap();
        let _ = paths_hmap.try_insert(path.clone(), HashMap::new()); // we can safely ignore the err

        // insert the callback and check for error
        let callbacks_hashmap = paths_hmap.get_mut(&path).unwrap();
        match callbacks_hashmap.try_insert(method, callback) {
            Ok(_) => {}
            Err(_) => {
                panic!("callback already exists");
            }
        }
    }

    pub fn listen(&self) {
        loop {
            log::info!("http server is listening");
            let mut req = self.server.recv().unwrap();
            let path_callbacks = self.listeners.as_ref().unwrap().get(req.url());

            let http_request = HttpRequest::from(&mut req);
            let response = match path_callbacks {
                Some(callbacks) => match callbacks.get(&http_request.method()) {
                    Some(callback) => {log::info!("http request received on {:?}", req.url()); callback(http_request)},
                    None => HttpResponse::from_bytes("invalid method"),
                },
                None => {
                    log::info!("path: {} not found", req.url());
                    // let mut buf = Vec::with_capacity(2048);
                    // req.as_reader().read_to_end(&mut buf).unwrap();

                    // let buf_arr: &[u8] = buf.as_ref();
                    // log::info!("{:?}", protocomm::SessionData::decode(buf_arr));

                    // let res = protocomm::SessionData{
                    //     sec_ver: SecSchemeVersion::SecScheme0.into(),
                    //     proto:  Some(Proto::Sec0(
                    //         Sec0Payload{
                    //             msg: Sec0MsgType::S0SessionResponse.into(),
                    //             payload: Some(sec0_payload::Payload::Sr(S0SessionResp{
                    //                 status: Status::Success.into()
                    //             }))
                    //         }
                    //     ))
                    // };

                    // let res = HttpResponse::from_bytes(res.encode_to_vec());

                    HttpResponse::from_bytes("404 not found".as_bytes())
                }
            };

            req.respond(tiny_http::Response::from_data(
                response.get_bytes_vectored(),
            ))
            .unwrap()
        }
    }
}
