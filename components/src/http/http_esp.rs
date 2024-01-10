use esp_idf_svc::{io::Write, http::server::{EspHttpConnection, Request}};

use crate::http::base::*;

impl From<esp_idf_svc::http::Method> for HttpMethod{
    fn from(value: esp_idf_svc::http::Method) -> Self {
        match value {
            esp_idf_svc::http::Method::Get => HttpMethod::GET,
            esp_idf_svc::http::Method::Post => HttpMethod::POST,
            _ => HttpMethod::OTHER
        }
    }
}

impl<'a> From<&Request<&mut EspHttpConnection<'a>>> for HttpRequest{
    fn from(req: &Request<&mut EspHttpConnection>) -> Self {
        Self {
            method: req.method().into()
        }
    }
}

impl<'a, U> HttpServer<'a, esp_idf_svc::http::server::EspHttpServer<'a>, U>
where
    U: Fn(HttpRequest) -> HttpResponse<'a> + Send + Sync + 'static
{
    pub fn new(config: &HttpConfiguration) -> anyhow::Result<Self>{
        let mut http_config = esp_idf_svc::http::server::Configuration::default();
        http_config.http_port = config.port;
        Ok(HttpServer{
            server: esp_idf_svc::http::server::EspHttpServer::new(&http_config).unwrap(),
            listeners: None
        }) 
    }

    pub fn add_listener(&mut self, path: &'a str, callback: U){
        self.server.fn_handler(path, esp_idf_svc::http::Method::Get, move |req|{
            let user_req = HttpRequest::from(&req);
            let user_response = callback(user_req);
            req.into_ok_response().unwrap().write_all(user_response.get_bytes()).unwrap();
            Ok(())
        }).unwrap();
    }

    pub fn listen(&self) -> anyhow::Result<()>{
        log::info!("http server is listening");
        loop{
            esp_idf_svc::hal::delay::Delay::new_default().delay_ms(1000)
        }
    }
}
