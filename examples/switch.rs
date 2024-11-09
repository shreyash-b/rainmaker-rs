use components::{
    persistent_storage::NvsPartition,
    wifi::{WifiClientConfig, WifiMgr},
    wifi_prov::{WiFiProvMgrBle, WifiProvBleConfig},
};
use rainmaker::{
    param::Param,
    device::{Device, DeviceType},
    error::RMakerError,
    node::Node,
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

fn main() -> Result<(), RMakerError> {
    initializse_logger();

    let rmaker = Rainmaker::init()?;
    let mut node = Node::new(rmaker.get_node_id());
    node.set_info(rainmaker::node::Info {
        name: "Switch Example Node".to_string(),
        fw_version: "v1.0".to_string(),
    });

    let mut switch_device = create_switch_device("Switch");
    switch_device.register_callback(Box::new(switch_cb));

    let wifi_arc_mutex = Arc::new(Mutex::new(WifiMgr::new()?));
    let nvs_partition = NvsPartition::new("nvs")?;

    let prov_config = WifiProvBleConfig {
        service_name: String::from("PROV_SERVICE"),
        ..Default::default()
    };
    let mut prov_mgr = WiFiProvMgrBle::new(
        wifi_arc_mutex.clone(),
        prov_config,
        nvs_partition,
        components::protocomm::ProtocommSecurity::new_sec1(None),
    )?;

    if let Some((ssid, password)) = prov_mgr.is_provisioned() {
        log::info!("Node already provisioned. Trying to connect");
        let mut wifi = wifi_arc_mutex.lock().unwrap();
        let config = WifiClientConfig {
            ssid,
            password,
            ..Default::default()
        };
        wifi.set_client_config(config)?;
        wifi.start()?;
        wifi.assured_connect();
        drop(prov_mgr);
    } else {
        log::info!("Node not provisioned. Starting WiFi provisioning.");
        rmaker.reg_user_mapping_ep(&mut prov_mgr);
        prov_mgr.start()?;
        prov_mgr.wait_for_provisioning();
    }

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
