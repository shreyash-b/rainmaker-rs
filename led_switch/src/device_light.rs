#[cfg(target_os = "espidf")]
use std::collections::HashMap;
#[cfg(target_os = "espidf")]
use std::sync::OnceLock;

use rainmaker::node::*;
#[cfg(target_os = "espidf")]
use serde_json::Value;
#[cfg(target_os = "espidf")]
use smart_leds::{hsv::hsv2rgb, RGB8};
use std::sync::Mutex;

#[cfg(target_os = "espidf")]
use smart_leds_trait::SmartLedsWrite;

#[cfg(target_os = "espidf")]
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrb24, LedPixelEsp32Rmt};

struct Hsv {
    hue: u16,
    sat: u8,
    val: u8,
}

static LIGHT_DATA: Mutex<(bool, Hsv)> = Mutex::new((
    false,
    Hsv {
        hue: 0,
        sat: 100,
        val: 5,
    },
)); // power and color

#[cfg(target_os = "espidf")]
pub(super) static LIGHT_DRIVER: OnceLock<Mutex<LedPixelEsp32Rmt<'_, RGB8, LedPixelColorGrb24>>> =
    OnceLock::new();

#[cfg(target_os = "espidf")]
pub fn handle_light_update(params: &HashMap<String, Value>) {
    if params.contains_key("Power")
        || params.contains_key("Hue")
        || params.contains_key("Saturation")
        || params.contains_key("Brightness")
    {
        let mut driver = LIGHT_DRIVER.get().unwrap().lock().unwrap();
        let mut curr_data = LIGHT_DATA.lock().unwrap();

        if let Some(power) = params.get("Power") {
            let power_val = power.as_bool().unwrap();
            curr_data.0 = power_val;
            if !power_val {
                let light = std::iter::repeat(hsv2rgb(smart_leds::hsv::Hsv::default())).take(1); // val should be 0
                driver.write(light).unwrap();
            }
        }

        if let Some(hue) = params.get("Hue") {
            curr_data.1.hue = hue.as_u64().unwrap() as u16;
        }

        if let Some(sat) = params.get("Saturation") {
            curr_data.1.sat = sat.as_u64().unwrap() as u8;
        }

        if let Some(brt) = params.get("Brightness") {
            curr_data.1.val = brt.as_u64().unwrap() as u8;
        }

        if curr_data.0 {
            // power
            let curr_hsv = smart_leds::hsv::Hsv {
                hue: map_to_255(curr_data.1.hue, 360.0),
                sat: map_to_255(curr_data.1.sat.into(), 100.0),
                val: map_to_255(curr_data.1.val.into(), 100.0),
            };

            log::info!("hue: {:?}", curr_hsv.hue);

            let light = std::iter::repeat(hsv2rgb(curr_hsv)).take(1);
            driver.write(light).unwrap();
        }
    }
}

pub fn create_light_device(name: &str) -> Device {
    let mut light_device = Device::new(name, DeviceType::Lightbulb, "Power", vec![]);
    let device_params = LIGHT_DATA.lock().unwrap();
    let power_param = Param::new_power("Power", device_params.0);
    let saturation_param = Param::new_satuation("Saturation", device_params.1.sat.into());
    let hue_param = Param::new_hue("Hue", device_params.1.hue.into());
    let brightness_param = Param::new_brighness("Brightness", device_params.1.val.into());

    light_device.add_param(power_param);
    light_device.add_param(hue_param);
    light_device.add_param(saturation_param);
    light_device.add_param(brightness_param);

    light_device
}

#[cfg(target_os = "espidf")]
fn map_to_255(val: u16, n: f32) -> u8 {
    let val_float = val as f32;
    let val = val_float * 255.0 / n;

    val as u8
}
