#![feature(lazy_cell)]

mod device_led;
mod device_light;

#[cfg(target_os="espidf")]
use components::{protocomm::ProtocommSecurity, wifi::WifiMgr};
use device_led::*;
use device_light::*;

#[cfg(target_os = "espidf")]
use esp_idf_svc::hal::{
    ledc::{self, LedcDriver, LedcTimerDriver},
    peripherals::Peripherals,
};
#[cfg(target_os = "espidf")]
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use std::collections::HashMap;
#[cfg(target_os = "espidf")]
use std::sync::{Arc, Mutex};

use rainmaker::{
    error::RMakerError, node::{Info, Node}, Rainmaker
};
#[cfg(target_os = "espidf")]
use rainmaker::wifi_prov::{WifiProvisioningConfig, WifiProvisioningMgr};
#[cfg(target_os = "espidf")]
use components::persistent_storage::NvsPartition;

use serde_json::Value;

fn led_cb(
    params: HashMap<String, Value>,
) {
    log::info!("led: {:?}", params);
    #[cfg(target_os="espidf")]
    device_led::handle_led_update(&params);
    rainmaker::report_params("LED", params);
}

fn light_cb(
    params: HashMap<String, Value>,
) {
    log::info!("light: {:?}", params);
    #[cfg(target_os="espidf")]
    device_light::handle_light_update(&params);
    rainmaker::report_params("Light", params);
}

fn initialize_logger() {
    #[cfg(target_os = "espidf")]
    esp_idf_svc::log::EspLogger::initialize_default();
    
    #[cfg(target_os = "linux")]
    simple_logger::SimpleLogger::default()
    .with_level(log::LevelFilter::Info)
    .init()
    .unwrap();
}

#[cfg(target_os="espidf")]
fn initialize_light_led_drivers(peripherals: Peripherals){
    let led_driver_local = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &ledc::config::TimerConfig::default(),
        )
        .unwrap(),
        peripherals.pins.gpio10,
    )
    .unwrap();
    let _ = LED_DRIVER.set(Mutex::new(led_driver_local));

    let light_driver_local =
        Ws2812Esp32Rmt::new(peripherals.rmt.channel0, peripherals.pins.gpio8).unwrap();
    let _ = LIGHT_DRIVER.set(Mutex::new(light_driver_local));
    
}

fn main() -> Result<(), RMakerError> {
    initialize_logger();
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging
    
    // NOTE FOR RUNNING CODE
    // you need your node to be previously provisioned for this code to work
    // for esp: perform the claiming using rainmaker cli [./rainmaker claim] and that should take care of rest
    // for linux: perform claiming for esp first to get the calimdata(only need to do this once).
    //      This will be stored in /home/{USERNAME}/.espressif/rainmaker/claim_data/{USER_ID}/{ESP_MAC}/
    //      Provide this path as RMAKER_CLAIMDATA_PATH environment variable before running rainmaker for the first time on linux
    //      [RMAKER_CLAIMDATA_PATH={YOUR_CLAIMDATA_PATH} cargo run_linux]
    //      This will fetch the relevant data and store using persistent storage
    
    let mut rmaker = Rainmaker::new().unwrap();

    #[cfg(target_os = "espidf")]
    let peripherals = Peripherals::take().unwrap();
    #[cfg(target_os = "espidf")]
    initialize_light_led_drivers(peripherals);
    
    #[cfg(target_os="espidf")]
    let wifi_arc_mutex = Arc::new(Mutex::new(WifiMgr::new()?));

    #[cfg(target_os = "espidf")]
    let nvs_partition = NvsPartition::new("nvs")?;

    #[cfg(target_os="espidf")]
    let mut prov_mgr = WifiProvisioningMgr::new(
        wifi_arc_mutex,
        WifiProvisioningConfig {
            device_name: "LED_LIGHT".to_owned(),
            scheme: rainmaker::wifi_prov::WifiProvScheme::SoftAP,
            security: ProtocommSecurity::default(),
        },
        nvs_partition.clone()
    );

    
    #[cfg(target_os="espidf")]
    match WifiProvisioningMgr::get_provisioned_creds(nvs_partition.clone()) {
        Some(_) => {
            log::warn!("Node already provisioned. Connecting");
            prov_mgr.connect().unwrap()
        }
        None => {
            log::warn!("Node not provisioned. Starting WiFi Provisioning");
            rmaker.reg_user_mapping_ep(&mut prov_mgr);
            prov_mgr.start().unwrap();
        }
    }
    

    let mut light_device = create_light_device("Light");
    light_device.register_callback(Box::new(light_cb));


    let mut led_device = create_led_device("LED");
    led_device.register_callback(Box::new(led_cb));

    let mut node = Node::new(
        rmaker.get_node_id(),
        "2019-02-27".to_string(),
        Info {},
        vec![],
    );
    node.add_device(light_device);
    node.add_device(led_device);
    rmaker.register_node(node);
    rmaker.start()?;
    drop(rmaker); // drop the lock so that callbacks can use it
    rainmaker::prevent_drop();

    Ok(())
}
