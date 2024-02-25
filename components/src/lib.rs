fn hello(name: &str) {
    log::info!("hello from {}", name);
}

pub fn say_hello() {
    #[cfg(target_os = "espidf")]
    hello("espidf");

    #[cfg(target_os = "linux")]
    hello("linux");
}
