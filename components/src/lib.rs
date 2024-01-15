#![feature(map_try_insert)] // used in http_linux.rs

pub mod http;

fn hello(name: &str){
    log::info!("hello from {}", name);
}

pub fn say_hello(){
    #[cfg(target_os="espidf")]
    hello("espidf");

    #[cfg(target_os="linux")]
    hello("linux");
}