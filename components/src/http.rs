use std::{collections::HashMap, net::{SocketAddr, IpAddr, Ipv4Addr}};

pub enum HttpMethod {
    GET,
    POST,
    OTHER
}

#[cfg(target_os="linux")]
impl From<&tiny_http::Method> for HttpMethod{
    fn from(inp: &tiny_http::Method) -> Self {
        match inp {
            tiny_http::Method::Get => HttpMethod::GET,
            tiny_http::Method::Post => HttpMethod::POST,
            _ => HttpMethod::OTHER
        }
    }
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
    server: T,
    listeners: HashMap<&'a str, U>
}

#[cfg(target_os="linux")]
impl<'a, U> HttpServer<'a, tiny_http::Server, U>
where
    U: Fn(HttpRequest) -> HttpResponse<'a>
{


    pub fn new(config: &HttpConfiguration) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>{
        Ok(HttpServer{
            server: tiny_http::Server::http(SocketAddr::new(config.addr, config.port))?,
            listeners: HashMap::new()
        })
    }

    pub fn add_listener(&mut self, path: &'a str, callback: U){
        self.listeners.insert(path, callback);

    }

    pub fn listen(&self) -> Result<(), Box<dyn std::error::Error>>{
        loop {
            let req = self.server.recv()?;
            let http_request = HttpRequest{
                method: req.method().into()
            };
            let req_callback = self.listeners.get(req.url());
            let response = match req_callback {
                Some(c) => c(http_request),
                None => HttpResponse::from_bytes("not found".as_bytes())
            };

            req.respond(tiny_http::Response::from_data(response.get_bytes()))?
        }
    }
}
