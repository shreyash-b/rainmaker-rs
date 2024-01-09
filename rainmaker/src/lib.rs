use components::say_hello;
use components::http::{HttpServer, HttpConfiguration, HttpResponse};

pub fn rainmaker_say_hello() {
    say_hello();
}

pub fn http_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{


    let config = HttpConfiguration::default();
    let mut server = HttpServer::new(&config)?;
    server.add_listener("/", |_req| -> HttpResponse {
        HttpResponse::from_bytes("root url".as_bytes())
    });
    server.listen().unwrap();

    Ok(())

}

pub fn rainmaker_init() {
    #[cfg(target_os = "espidf")]
    esp_idf_svc::log::EspLogger::initialize_default();

    #[cfg(target_os = "linux")]
    simple_logger::SimpleLogger::new().init().unwrap();
}


