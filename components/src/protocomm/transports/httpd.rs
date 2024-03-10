use crate::protocomm::protocomm_req_handler;
use crate::protocomm::transports::TransportTrait;
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::http::{HttpConfiguration, HttpMethod, HttpRequest, HttpResponse, HttpServer};

pub struct TransportHttpd<'a> {
    http_server: Arc<Mutex<HttpServer<'a>>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> TransportHttpd<'a> {
    pub fn new() -> Self {
        let http_server = HttpServer::new(&HttpConfiguration::default()).unwrap();
        Self {
            http_server: Arc::new(Mutex::new(http_server)),
            phantom: PhantomData::default(),
        }
    }

}

impl<'a> TransportTrait<'a> for TransportHttpd<'a> {
    fn add_endpoint<T>(&self, ep_name: &str, cb: T)
    where
        T: Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static,
    {
        let mut http_server = self.http_server.lock().unwrap();
        let ep = "/".to_string() + &ep_name;
        http_server.add_listener(
            ep,
            HttpMethod::POST,
            Box::new(move |req| -> HttpResponse { http_callback(req, cb.borrow()) }),
        );
    }
}

fn http_callback<T>(mut req: HttpRequest, cb: T) -> HttpResponse
where
    T: Fn(String, Vec<u8>) -> Vec<u8>,
{
    let url = req.url();
    let data = req.data();
    let ep = url.split_at(1).1.to_owned();

    let data_ret = protocomm_req_handler(ep, data, cb);

    HttpResponse::from_bytes(data_ret)
}
