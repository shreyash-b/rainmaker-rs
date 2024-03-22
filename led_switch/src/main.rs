#![feature(never_type)]

mod device_led;
mod device_light;
use components::protocomm::ProtocommSecurity;
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
use std::sync::Mutex;

use rainmaker::{
    error::RMakerError,
    node::{Info, Node},
    Rainmaker,
};
use serde_json::Value;

fn led_cb(
    params: HashMap<String, /* ParamDataType */ Value>,
    driver: &LedDriverType,
    rmaker: &Mutex<Rainmaker<'static>>,
) {
    log::info!("led: {:?}", params);
    device_led::handle_led_update(params, driver, rmaker)
}

fn light_cb(
    params: HashMap<String, /* ParamDataType */ Value>,
    driver: &LightDriverType,
    rmaker: &Mutex<Rainmaker<'static>>,
) {
    log::info!("light: {:?}", params);
    device_light::handle_light_update(params, driver, rmaker);
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

    let led_driver: &device_led::LedDriverType;
    let light_driver: &device_light::LightDriverType;
    let _st = "string";

    #[cfg(target_os = "espidf")]
    {
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

        let light_driver_local =
            Ws2812Esp32Rmt::new(peripherals.rmt.channel0, peripherals.pins.gpio8).unwrap();

        led_driver = Box::leak(Box::new(Mutex::new(led_driver_local)));
        light_driver = Box::leak(Box::new(Mutex::new(light_driver_local)));
    }

    #[cfg(target_os = "linux")]
    {
        led_driver = Box::leak(Box::new(()));
        light_driver = Box::leak(Box::new(()));
    }

    let mut light_device = create_light_device("Light");

    let rmaker_mutex = Box::leak(Box::new(Mutex::new(Rainmaker::new()?))); // needed just to keep compiler happy

    #[allow(clippy::mut_mutex_lock)]
    // clippy suggests using get_mut() here but we don't want to do that
    let mut rmaker = rmaker_mutex.lock().unwrap();

    light_device.register_callback(Box::new(|params| {
        light_cb(params, light_driver, rmaker_mutex)
    }));

    rmaker.init();

    let mut led_device = create_led_device("LED");
    led_device.register_callback(Box::new(|params| led_cb(params, led_driver, rmaker_mutex)));

    let mut node = Node::new(
        rmaker.get_node_id(),
        "2019-02-27".to_string(),
        Info {},
        vec![],
    );
    node.add_device(light_device);
    node.add_device(led_device);
    rmaker.register_node(node);
    rmaker.init_wifi(ProtocommSecurity::new_sec1(Some("abcd1234".to_string())))?; // hardcoded
    rmaker.start()?;
    drop(rmaker); // drop the lock so that callbacks can use it
    rainmaker::prevent_drop();

    Ok(())
}
