use rainmaker::{wifi_prov::WifiProvisioningConfig, Rainmaker};

fn main(){
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging

    let mut rmaker = Rainmaker::new();
    rmaker.init();
    rmaker.init_prov(WifiProvisioningConfig{
        device_name: "RMAKER_PROV123".to_string(),
        ..Default::default()
    });
    rmaker.start_prov();

    rainmaker::prevent_drop();
}
