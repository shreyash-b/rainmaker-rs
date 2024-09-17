use components::{persistent_storage::NvsPartition, wifi::WifiMgr};
use rainmaker::{
    error::RMakerError,
    node::{Device, Info, Node, Param},
    wifi_prov::{WifiProvisioningConfig, WifiProvisioningMgr},
    Rainmaker,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

fn initializse_logger() {
    #[cfg(target_os = "linux")]
    simple_logger::init_with_level(log::Level::Info).unwrap();

    #[cfg(target_os = "espidf")]
    esp_idf_svc::log::EspLogger::initialize_default();
}

fn create_switch_device(device_name: &str) -> Device {
    let mut switch_dev = Device::new(
        device_name,
        rainmaker::node::DeviceType::Switch,
        "Power",
        vec![],
    );

    let power_param = Param::new_power("Power", false);
    switch_dev.add_param(power_param);

    switch_dev
}

fn switch_cb(params: HashMap<String, Value>) {
    log::info!("Received update: {:?}", params);
    rainmaker::report_params("Switch", params);
}

fn main() -> Result<(), RMakerError> {
    initializse_logger();

    let mut rmaker = Rainmaker::new()?;
    let mut node = Node::new(
        rmaker.get_node_id(),
        "2019-02-27".to_string(),
        Info {},
        vec![],
    );
    let mut switch_device = create_switch_device("Switch");
    switch_device.register_callback(Box::new(switch_cb));

    let wifi_arc_mutex = Arc::new(Mutex::new(WifiMgr::new()?));
    let nvs_partition = NvsPartition::new("nvs")?;

    let provisioning_config = WifiProvisioningConfig {
        device_name: "Switch".to_string(),
        ..Default::default()
    };
    let mut prov_mgr =
        WifiProvisioningMgr::new(wifi_arc_mutex, provisioning_config, nvs_partition.clone());

    if WifiProvisioningMgr::get_provisioned_creds(nvs_partition.clone()).is_some() {
        log::info!("Device is already provisioned");
    } else {
        log::info!("Device is not provisioned. Starting provisioning...");
        rmaker.reg_user_mapping_ep(&mut prov_mgr);
        prov_mgr.start()?;
    }

    while WifiProvisioningMgr::get_provisioned_creds(nvs_partition.clone()).is_none() {
        log::info!("Waiting for provisioning...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    prov_mgr.connect()?;

    node.add_device(switch_device);

    rmaker.register_node(node);
    rmaker.start()?;

    log::info!("Rainmaker agent is started");

    rainmaker::prevent_drop();

    Ok(())
}
