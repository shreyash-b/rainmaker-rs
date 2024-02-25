use rainmaker::{node::*, Rainmaker};
use serde_json::Value;
use std::collections::HashMap;

#[cfg(target_os = "espidf")]
use esp_idf_svc::hal::ledc::LedcDriver;
use std::sync::Mutex;

static LED_DATA: Mutex<(bool, u32)> = Mutex::new((false, 0)); // power, brightness

#[cfg(target_os = "espidf")]
pub type LedDriverType<'a> = Mutex<LedcDriver<'a>>;
#[cfg(target_os = "linux")]
pub type LedDriverType<'a> = ();

#[cfg(target_os = "espidf")]
pub fn handle_led_update(params: HashMap<String, Value>, driver: &LedDriverType, rmaker: &Mutex<Rainmaker<'static>>) {
    if params.get("Power").is_some() || params.get("Brightness").is_some() {
        let mut driver = driver.lock().unwrap();
        let mut curr_data = LED_DATA.lock().unwrap();

        match params.get("Power") {
            Some(power) => {
                let power_val = power.as_bool().unwrap();
                curr_data.0 = power_val;
                if power_val == false {
                    driver.set_duty(0).unwrap();
                }
            }
            None => {}
        };

        match params.get("Brightness") {
            Some(brt) => {
                let brt_val = brt.as_f64().unwrap();
                let brt_val = brt_val * 255.0 / 100.0;
                curr_data.1 = brt_val as u32;
            }
            None => {}
        };

        if curr_data.0 == true {
            // power
            driver.set_duty(curr_data.1).unwrap();
        }

        report_params(params, rmaker)
    }
}

#[cfg(target_os = "linux")]
pub fn handle_led_update(_params: HashMap<String, Value>, _driver: &LedDriverType, rmaker: &Mutex<Rainmaker<'static>>) {
    report_params(_params, rmaker)
}

pub fn create_led_device(name: &str) -> Device {
    let mut led_device = Device::new(name, DeviceType::Light, "Power", vec![]);
    let device_params = LED_DATA.lock().unwrap();
    let power_param = Param::new_power("Power", device_params.0);
    let brightness_param = Param::new_brighness("Brightness", device_params.1);

    led_device.add_param(power_param);
    led_device.add_param(brightness_param);

    led_device
}

fn report_params(params: HashMap<String, Value>, rmaker: &Mutex<Rainmaker<'static>>){
    let rmaker_lock = rmaker.lock().unwrap();
    rmaker_lock.report_params("LED", params);
    drop(rmaker_lock);

}