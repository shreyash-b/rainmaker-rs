use rainmaker::{error::RMakerError, node::{DeviceType, Devices, Info, Node, Params}, wifi_prov::WifiProvisioningConfig, Rainmaker};

fn main() -> Result<(), RMakerError>{
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging

    let mut led_device = Devices::new("LED2", DeviceType::Lightbulb, "Power", vec![], vec![]);
    let power_param = Params::new("Power", "bool", vec!["read".to_string(), "write".to_string()], "esp.ui.toggle");
    let brightness_param = Params::new("Brightness", "int", vec!["read".to_string(), "write".to_string()], "esp.ui.slider");
    led_device.add_param(power_param);
    led_device.add_param(brightness_param);

    let mut node = Node::new("58CF79DAC1E4".to_string(), "2019-02-27".to_string(), Info{}, vec![], vec![]);

    node.add_device(led_device);

    let mut rmaker = Rainmaker::new()?;
    // rmaker.init_prov(WifiProvisioningConfig{
    //     device_name: "1234".to_string(),
    //     ..Default::default()
    // });
    // rmaker.start_prov();
    rmaker.register_node(node);
    rmaker.init();
    rmaker.init_wifi();
    rmaker.start()?;
    rainmaker::prevent_drop();
    
    Ok(())
}
