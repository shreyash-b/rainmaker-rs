fn main() {
    let target_os = std::env::var_os("CARGO_CFG_TARGET_OS").expect("");
    if target_os == "espidf" {
        embuild::espidf::sysenv::output();
    }
}
