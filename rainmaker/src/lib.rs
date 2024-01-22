use components::http::{HttpConfiguration, HttpResponse, HttpServer};
use components::say_hello;

#[cfg(target_os="espidf")]
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripherals::Peripherals,
    wifi::{ClientConfiguration, Configuration, EspWifi},
};

pub fn rainmaker_say_hello() {
    say_hello();
}

#[cfg(target_os = "espidf")]
// workaround function to connect wifi on esp
// will be removed once wifi provisoining is implemented
pub fn connect_wifi<'a>(ssid: &'a str, key: &'a str) -> EspWifi<'a> {
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    // let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = EspWifi::new(peripherals.modem, sysloop, None).unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        auth_method: esp_idf_svc::wifi::AuthMethod::None,
        bssid: None,
        password: key.into(),
        channel: None,
    })).expect("unable to set wifi config");

    wifi.start().expect("unable to start wifi");
    wifi.connect().expect("unable to connect wifi");
    log::info!("wifi is connected...");

    wifi
}

pub fn http_server() -> anyhow::Result<()> {
    #[cfg(target_os = "espidf")]
    let _wifi = connect_wifi("Connecting...", "0000@1111");

    let config = HttpConfiguration::default();
    let mut server = HttpServer::new(&config)?;
    server.add_listener("/", |_req| -> HttpResponse {
        std::thread::sleep(std::time::Duration::from_millis(1500)); // for testing concurrency
        HttpResponse::from_bytes("root url".as_bytes())
    });
    server.listen()
}

pub fn rainmaker_init() {
    #[cfg(target_os = "espidf")]
    esp_idf_svc::log::EspLogger::initialize_default();

    #[cfg(target_os = "linux")]
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
}
