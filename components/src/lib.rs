use log::{trace, debug, info, warn, error};

// type definitions
struct Wifi {

}

#[cfg(target_os="linux")]
// write linux implementation of wifi here
impl Wifi{

}

#[cfg(target_os="espidf")]
// write esp implementation of wifi here
impl Wifi{

}
