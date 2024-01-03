#[cfg(target_os="espidf")]
pub fn say_hello(){
    println!("hello from esp");
}

#[cfg(target_os="linux")]
pub fn say_hello(){
    println!("hello from linux");
}

