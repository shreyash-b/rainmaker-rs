use components::say_hello;

pub fn rainmaker_say_hello() {
    say_hello();
}

pub fn rainmaker_init() {
    #[cfg(target_os = "espidf")]
    esp_idf_svc::log::EspLogger::initialize_default();

    #[cfg(target_os = "linux")]
    simple_logger::SimpleLogger::new().init().unwrap();
}
