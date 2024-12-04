use anyhow::Result;
use examples::{connect_wifi, initializse_logger};
use rainmaker::components::wifi::WifiMgr;
use rainmaker::{
    device::{Device, DeviceType},
    node::Node,
    param::Param,
    Rainmaker,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

fn create_switch_device(device_name: &str) -> Device {
    let mut switch_dev = Device::new(device_name, DeviceType::Switch);

    let power_param = Param::new_power("Power", false);

    switch_dev.add_param(power_param);
    switch_dev.set_primary_param("Power");

    switch_dev
}

fn switch_cb(params: HashMap<String, Value>) {
    log::info!("Received update: {:?}", params);
    log::info!("Reporting: {:?}", params);
    rainmaker::report_params("Switch", params);
}

fn main() -> Result<()> {
    initializse_logger();

    let rmaker = Rainmaker::init()?;
    let mut node = Node::new(rmaker.get_node_id());
    node.set_info(rainmaker::node::Info {
        name: "Switch Example Node".to_string(),
        fw_version: "v1.0".to_string(),
    });

    let mut switch_device = create_switch_device("Switch");
    switch_device.register_callback(Box::new(switch_cb));

    // Declare it here since we want wifi to be connected after connect_wifi returns
    let wifi_arc_mutex = Arc::new(Mutex::new(WifiMgr::new()?));
    connect_wifi(rmaker, wifi_arc_mutex.clone())?;

    log::info!("WiFi connected successfully");

    node.add_device(switch_device);

    rmaker.register_node(node);
    rmaker.start()?;

    log::info!("Rainmaker agent is started");

    // Inorder to prevent rmaker from drop
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
