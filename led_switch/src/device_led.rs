use rainmaker::node::*;
#[cfg(target_os = "espidf")]
use serde_json::Value;
#[cfg(target_os = "espidf")]
use std::{collections::HashMap, sync::LazyLock};

#[cfg(target_os = "espidf")]
use esp_idf_svc::hal::ledc::LedcDriver;
#[cfg(target_os = "espidf")]
use std::sync::{Mutex, OnceLock};

#[cfg(target_os = "espidf")]
static LED_DATA: LazyLock<Mutex<(bool, u32)>> = LazyLock::new(|| Mutex::new((false, 0))); // power, brightness

#[cfg(target_os = "espidf")]
pub(super) static LED_DRIVER: OnceLock<Mutex<LedcDriver<'_>>> = OnceLock::new();

#[cfg(target_os = "espidf")]
pub fn handle_led_update(params: &HashMap<String, Value>) {
    if params.get("Power").is_some() || params.get("Brightness").is_some() {
        let mut driver = LED_DRIVER.get().unwrap().lock().unwrap();
        let mut curr_data = LED_DATA.lock().unwrap();

        if let Some(power) = params.get("Power") {
            let power_val = power.as_bool().unwrap();
            curr_data.0 = power_val;
            if !power_val {
                driver.set_duty(0).unwrap();
            }
        };

        if let Some(brt) = params.get("Brightness") {
            let brt_val = brt.as_f64().unwrap();
            let brt_val = brt_val * 255.0 / 100.0;
            curr_data.1 = brt_val as u32;
        };

        if curr_data.0 {
            // power
            driver.set_duty(curr_data.1).unwrap();
        }
    }
}

pub fn create_led_device(name: &str) -> Device {
    let mut led_device = Device::new(name, DeviceType::Light, "Power", vec![]);
    let power_param = Param::new_power("Power", false);
    let brightness_param = Param::new_brighness("Brightness", 0);

    led_device.add_param(power_param);
    led_device.add_param(brightness_param);

    led_device
}
