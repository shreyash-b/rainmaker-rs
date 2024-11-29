//! Common utilities used across multiple examples

use std::sync::{Arc, Mutex};

use anyhow::Result;
use components::{
    persistent_storage::NvsPartition,
    wifi::{WifiClientConfig, WifiMgr},
    wifi_prov::{WiFiProvMgrBle, WifiProvBleConfig},
};
use rainmaker::Rainmaker;

pub fn initializse_logger() {
    #[cfg(target_os = "linux")]
    simple_logger::init_with_level(log::Level::Info).unwrap();

    #[cfg(target_os = "espidf")]
    esp_idf_svc::log::EspLogger::initialize_default();
}

pub fn connect_wifi(
    rmaker: &Rainmaker,
    wifi_arc_mutex: Arc<Mutex<WifiMgr<'static>>>,
) -> Result<()> {
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
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::new(30, 0));
            log::info!("Stopping WiFi provisioning");
            drop(prov_mgr);
        });
    }

    Ok(())
}
