use rainmaker::{error::RMakerError, node::{DeviceType, Devices, Info, Node, Params}, Rainmaker};

fn main() -> Result<(), RMakerError>{
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging

    // NOTE FOR RUNNING CODE
    // you need your node to be previously provisioned for this code to work
    // for esp: perform the claiming using rainmaker cli [./rainmaker claim] and that should take care of rest
    // for linux: perform claiming for esp first to get the calimdata(only need to do this once). 
    //      This will be stored in /home/{USERNAME}/.espressif/rainmaker/claim_data/{USER_ID}/{ESP_MAC}/
    //      Provide this path as RMAKER_CLAIMDATA_PATH environment variable before running rainmaker for the first time on linux 
    //      [RMAKER_CLAIMDATA_PATH={YOUR_CLAIMDATA_PATH} cargo run_linux]
    //      This will fetch the relevant data and store using persistent storage

    let mut led_device = Devices::new("LED", DeviceType::Lightbulb, "Power", vec![], vec![]);
    let power_param = Params::new("Power", "bool", vec!["read".to_string(), "write".to_string()], "esp.ui.toggle");
    let brightness_param = Params::new("Brightness", "int", vec!["read".to_string(), "write".to_string()], "esp.ui.slider");
    led_device.add_param(power_param);
    led_device.add_param(brightness_param);
    
    
    let mut rmaker = Rainmaker::new()?;
    let mut node = Node::new(rmaker.get_node_id_stored(), "2019-02-27".to_string(), Info{}, vec![], vec![]);
    
    node.add_device(led_device);
    rmaker.register_node(node);
    rmaker.init();
    rmaker.init_wifi();
    rmaker.start()?;
    rainmaker::prevent_drop();
    
    Ok(())
}
