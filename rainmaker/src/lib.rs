pub mod wifi_prov;
use components::{say_hello, wifi::WifiMgr, http::{HttpServer, HttpConfiguration}};
// use wifi_prov::WifiProvisioningConfig;


pub fn rainmaker_say_hello() {
    say_hello();
}



pub fn rainmaker_init() {
    #[cfg(target_os = "espidf")]
    esp_idf_svc::log::EspLogger::initialize_default();

    #[cfg(target_os = "linux")]
    simple_logger::SimpleLogger::new().init().unwrap();
}

// pub fn prov_test(){
//     let wifi = WifiMgr::new();
//     let http_server = HttpServer::new(&HttpConfiguration::default()).unwrap();

//     let provisioning_config = WifiProvisioningConfig{
//         device_name: "RMaker_123".into(),
//         ..Default::default()
//     };
//     let mut prov_mgr = wifi_prov::WifiProvisioningMgr::new(wifi, http_server, provisioning_config);

//     prov_mgr.start();
//     prevent_drop();
// }

pub fn prevent_drop(){
    // eat 5-star, do nothing
    // to avoid variables from dropping
    loop{
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}