use crate::http::{HttpConfiguration, HttpMethod, HttpRequest, HttpResponse, HttpServer};
use crate::protocomm::{
    protocomm_req_handler, EndpointType, ProtocommCallbackType, ProtocommSecurity,
};
use std::sync::Arc;

pub(crate) struct TransportHttpd {
    http_server: HttpServer,
}

impl TransportHttpd {
    pub(crate) fn new(config: HttpConfiguration) -> Self {
        let http_server = HttpServer::new(config).unwrap();
        Self { http_server }
    }
}

impl TransportHttpd {
    pub(crate) fn add_endpoint(
        &mut self,
        ep_name: &str,
        cb: ProtocommCallbackType,
        ep_type: EndpointType,
        sec: Arc<ProtocommSecurity>,
    ) {
        let http_server = &mut self.http_server;
        let ep = "/".to_string() + ep_name;
        http_server.add_listener(
            ep,
            HttpMethod::POST,
            Box::new(move |req| -> HttpResponse { http_callback(req, &cb, &ep_type, &sec) }),
        );
    }
}

fn http_callback(
    mut req: HttpRequest,
    cb: &ProtocommCallbackType,
    ep_type: &EndpointType,
    sec: &Arc<ProtocommSecurity>,
) -> HttpResponse {
    let url = req.url();
    let data = req.data();
    let ep = url.split_at(1).1;

    let data_ret = protocomm_req_handler(ep, &data, cb, ep_type, sec);

    HttpResponse::from_bytes(data_ret)
}
