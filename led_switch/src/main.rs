#![feature(never_type)]

mod device_led;
mod device_light;
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
use std::sync::Mutex;

use rainmaker::{
    error::RMakerError,
    node::{Info, Node},
    Rainmaker,
};
use serde_json::Value;

fn led_cb(params: HashMap<String, /* ParamDataType */ Value>, driver: &LedDriverType) {
    log::info!("led: {:?}", params);
    device_led::handle_led_update(params, driver)
}

fn light_cb(params: HashMap<String, /* ParamDataType */ Value>, driver: &LightDriverType) {
    log::info!("light: {:?}", params);
    device_light::handle_light_update(params, driver);
}

fn main() -> Result<(), RMakerError> {
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging

    // NOTE FOR RUNNING CODE
    // you need your node to be previously provisioned for this code to work
    // for esp: perform the claiming using rainmaker cli [./rainmaker claim] and that should take care of rest
    // for linux: perform claiming for esp first to get the calimdata(only need to do this once).
    //      This will be stored in /home/{USERNAME}/.espressif/rainmaker/claim_data/{USER_ID}/{ESP_MAC}/
    //      Provide this path as RMAKER_CLAIMDATA_PATH environment variable before running rainmaker for the first time on linux
    //      [RMAKER_CLAIMDATA_PATH={YOUR_CLAIMDATA_PATH} cargo run_linux]
    //      This will fetch the relevant data and store using persistent storage

    #[cfg(target_os = "espidf")]
    let peripherals = Peripherals::take().unwrap();

    #[cfg(target_os = "espidf")]
    let led_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &ledc::config::TimerConfig::default(),
        )
        .unwrap(),
        peripherals.pins.gpio2,
    )
    .unwrap();

    #[cfg(target_os = "espidf")]
    let led_driver = Box::leak(Box::new(Mutex::new(led_driver)));

    #[cfg(target_os = "linux")]
    let led_driver = Box::leak(Box::new(()));

    #[cfg(target_os = "espidf")]
    let light_driver =
        Ws2812Esp32Rmt::new(peripherals.rmt.channel0, peripherals.pins.gpio8).unwrap();

    #[cfg(target_os = "espidf")]
    let light_driver = Box::leak(Box::new(Mutex::new(light_driver)));

    #[cfg(target_os = "linux")]
    let light_driver = Box::leak(Box::new(()));

    let mut light_device = create_light_device("Light");

    light_device.register_callback(Box::new(|params| light_cb(params, light_driver)));

    let mut rmaker: Rainmaker<'_> = Rainmaker::new()?;
    rmaker.init();

    let mut led_device = create_led_device("LED");
    led_device.register_callback(Box::new(|params| led_cb(params, led_driver)));

    let mut node = Node::new(
        rmaker.get_node_id(),
        "2019-02-27".to_string(),
        Info {},
        vec![],
    );
    node.add_device(light_device);
    node.add_device(led_device);
    rmaker.register_node(node);
    rmaker.init_wifi()?;
    rmaker.start()?;
    rainmaker::prevent_drop();

    Ok(())
}
