use std::collections::HashMap;

use rainmaker::node::*;
use serde_json::Value;
#[cfg(target_os = "espidf")]
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    RGB8,
};
#[cfg(target_os = "espidf")]
use std::sync::Mutex;

#[cfg(target_os = "espidf")]
use smart_leds_trait::SmartLedsWrite;

#[cfg(target_os = "espidf")]
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrb24, LedPixelEsp32Rmt};

#[cfg(target_os = "espidf")]
static LIGHT_DATA: Mutex<(bool, Hsv)> = Mutex::new((
    false,
    Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    },
)); // power and color

#[cfg(target_os = "espidf")]
pub type LightDriverType<'a> = Mutex<LedPixelEsp32Rmt<'a, RGB8, LedPixelColorGrb24>>;

#[cfg(target_os = "linux")]
pub type LightDriverType = ();

#[cfg(target_os = "espidf")]
pub fn handle_light_update(params: HashMap<String, Value>, driver: &LightDriverType) {
    if params.contains_key("Power")
        || params.contains_key("Hue")
        || params.contains_key("Saturation")
        || params.contains_key("Brightness")
    {
        let mut driver = driver.lock().unwrap();
        let mut curr_data = LIGHT_DATA.lock().unwrap();

        match params.get("Power") {
            Some(power) => {
                let power_val = power.as_bool().unwrap();
                curr_data.0 = power_val;
                if power_val == false {
                    let light = std::iter::repeat(hsv2rgb(Hsv::default())).take(1); // val should be 0
                    driver.write(light).unwrap();
                }
            }
            None => {}
        }

        match params.get("Hue") {
            Some(hue) => {
                let hue_val = hue.as_f64().unwrap();
                let hue_val = hue_val / 360.0 * 255.0;

                curr_data.1.hue = hue_val as u8;
            }
            None => {}
        }

        match params.get("Saturation") {
            Some(sat) => {
                let sat_val = sat.as_f64().unwrap();
                let sat_val = sat_val / 100.0 * 255.0;

                curr_data.1.sat = sat_val as u8;
            }
            None => {}
        }

        match params.get("Brightness") {
            Some(brt) => {
                let brt_val = brt.as_f64().unwrap();
                let brt_val = brt_val / 100.0 * 255.0;

                curr_data.1.val = brt_val as u8;
            }
            None => {}
        }

        if curr_data.0 == true {
            // power
            let light = std::iter::repeat(hsv2rgb(curr_data.1)).take(1);
            driver.write(light).unwrap();
        }
    }
}

#[cfg(target_os = "linux")]
pub fn handle_light_update(_params: HashMap<String, Value>, _driver: &LightDriverType) {}

pub fn create_light_device(name: &str) -> Device {
    let mut light_device = Device::new(name, DeviceType::Lightbulb, "Power", vec![]);
    let power_param = Param::new_power("Power");
    let saturation_param = Param::new_satuation("Saturation");
    let hue_param = Param::new_hue("Hue");
    let brightness_param = Param::new_brighness("Brightness");

    light_device.add_param(power_param);
    light_device.add_param(hue_param);
    light_device.add_param(saturation_param);
    light_device.add_param(brightness_param);

    light_device
}
