fn main(){
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging

    rainmaker::rainmaker_init();
    rainmaker::rainmaker_say_hello();
    rainmaker::wifi_prov::prov_test();

    rainmaker::prevent_drop();
}
