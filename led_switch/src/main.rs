#![feature(never_type)]

mod devices;
use devices::*;

#[cfg(target_os="espidf")]
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    RGB8
};

#[cfg(target_os="espidf")]
use smart_leds_trait::SmartLedsWrite;

#[cfg(target_os="espidf")]
use ws2812_esp32_rmt_driver::{Ws2812Esp32Rmt, LedPixelEsp32Rmt, driver::color::LedPixelColorGrb24};
use std::collections::HashMap;

#[cfg(target_os="espidf")]
use esp_idf_svc::hal::{
    peripherals::Peripherals,
    ledc::{LedcDriver, LedcTimerDriver},
    *
};

#[cfg(target_os="espidf")]
use std::sync::Mutex;

use rainmaker::{error::RMakerError, node::{Info, Node}, Rainmaker};
use serde_json::Value;

#[cfg(target_os="espidf")]
type LedDriverType<'a> = Mutex<LedcDriver<'a>>;

#[cfg(target_os="linux")]
type LedDriverType<'a> = ();

#[cfg(target_os="espidf")]
type LightDriverType<'a> = Mutex<LedPixelEsp32Rmt<'a,  RGB8, LedPixelColorGrb24>>;

#[cfg(target_os="linux")]
type LightDriverType = ();

#[cfg(target_os="espidf")]
static LIGHT_DATA: Mutex<(bool, Hsv)> = Mutex::new((false, Hsv{hue: 0, sat: 255, val: 255})); // power and color

#[cfg(target_os="espidf")]
static LED_DATA: Mutex<(bool, u32)> = Mutex::new((false, 0)); // power, brightness


#[cfg(target_os="espidf")]
fn handle_light_update( params: HashMap<String, Value>, driver: &LightDriverType){
    if params.contains_key("Power") || params.contains_key("Hue") || params.contains_key("Saturation") || params.contains_key("Brightness") {
        let mut driver = driver.lock().unwrap();
        let mut curr_data = LIGHT_DATA.lock().unwrap();

        match params.get("Power"){
            Some(power) => {
                let power_val = power.as_bool().unwrap();
                curr_data.0 = power_val;
                if power_val == false {
                    let light = std::iter::repeat(hsv2rgb(Hsv::default())).take(1); // val should be 0
                    driver.write(light).unwrap();
                }
            }, 
            None => {}
        }

        match params.get("Hue"){
            Some(hue) => {
                let hue_val = hue.as_f64().unwrap();
                let hue_val = hue_val / 360.0  * 255.0;
                
                curr_data.1.hue = hue_val as u8;
            }, 
            None => {}
        }
        
        match params.get("Saturation"){
            Some(sat) => {
                let sat_val = sat.as_f64().unwrap();
                let sat_val = sat_val / 100.0 * 255.0;
    
                curr_data.1.sat = sat_val as u8;
            }, 
            None => {}
        }
    
        match params.get("Brightness"){
            Some(brt) => {
                let brt_val = brt.as_f64().unwrap();
                let brt_val = brt_val / 100.0 * 255.0;
    
                curr_data.1.val = brt_val as u8;
            }, 
            None => {}
        }
        
        if curr_data.0 == true { // power
            let light = std::iter::repeat(hsv2rgb(curr_data.1)).take(1);
            driver.write(light).unwrap();
        } 
    }
    
}

#[cfg(target_os="linux")]
fn handle_light_update(_params: HashMap<String, Value>, _driver: &LightDriverType){

}

fn light_cb(params: HashMap<String, /* ParamDataType */ Value>, driver: &LightDriverType){
    log::info!("light: {:?}", params);
    handle_light_update(params, driver);
}

#[cfg(target_os="espidf")]
fn handle_led_update(params: HashMap<String, Value>, driver: &LedDriverType){
    if params.get("Power").is_some() || params.get("Brightness").is_some() {
        let mut driver = driver.lock().unwrap();
        let mut curr_data = LED_DATA.lock().unwrap();

        match params.get("Power"){
            Some(power) => {
                let power_val = power.as_bool().unwrap();
                curr_data.0 = power_val;
                if power_val == false {
                    driver.set_duty(0).unwrap();
                }
            },
            None => {}
        };
        
        match params.get("Brightness"){
            Some(brt) => {
                let brt_val = brt.as_f64().unwrap();
                let brt_val = brt_val * 255.0 / 100.0 ;
                curr_data.1 = brt_val as u32;
            },
            None => {}
        };

        if curr_data.0 == true{ // power
            driver.set_duty(curr_data.1).unwrap();
        }
    }
    
}

#[cfg(target_os="linux")]
fn handle_led_update(_params: HashMap<String, Value>, _driver: &LedDriverType){
    // todo
}

fn led_cb(params: HashMap<String, /* ParamDataType */ Value>, driver: &LedDriverType){
    log::info!("led: {:?}", params);
    handle_led_update(params, driver)
    
}

fn main() -> Result<(), RMakerError>{
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging
    
    // NOTE FOR RUNNING CODE
    // you need your node to be previously provisioned for this code to work
    // for esp: perform the claiming using rainmaker cli [./rainmaker claim] and that should take care of rest
    // for linux: perform claiming for esp first to get the calimdata(only need to do this once). 
    //      This will be stored in /home/{USERNAME}/.espressif/rainmaker/claim_data/{USER_ID}/{ESP_MAC}/
    //      Provide this path as RMAKER_CLAIMDATA_PATH environment variable before running rainmaker for the first time on linux 
    //      [RMAKER_CLAIMDATA_PATH={YOUR_CLAIMDATA_PATH} cargo run_linux]
    //      This will fetch the relevant data and store using persistent storage
    
    #[cfg(target_os="espidf")]
    let peripherals = Peripherals::take().unwrap();
    
    #[cfg(target_os="espidf")]
    let led_driver = LedcDriver::new(
        peripherals.ledc.channel0, 
        LedcTimerDriver::new(
            peripherals.ledc.timer0, 
            &ledc::config::TimerConfig::default()
        ).unwrap(),
        peripherals.pins.gpio2
    ).unwrap();

    #[cfg(target_os="espidf")]
    let led_driver = Box::leak(Box::new(Mutex::new(led_driver)));
    
    #[cfg(target_os="linux")]
    let led_driver = Box::leak(Box::new(()));

    #[cfg(target_os="espidf")]
    let light_driver = Ws2812Esp32Rmt::new(
        peripherals.rmt.channel0, 
        peripherals.pins.gpio8
    ).unwrap();

    #[cfg(target_os="espidf")]
    let light_driver = Box::leak(Box::new(Mutex::new(light_driver)));

    #[cfg(target_os="linux")]
    let light_driver = Box::leak(Box::new(()));

    let mut light_device = create_light_device("Light");

    light_device.register_callback(Box::new(|params|{light_cb(params, light_driver)}));

    let mut rmaker: Rainmaker<'_> = Rainmaker::new()?;
    rmaker.init();
    
    let mut led_device = create_led_device("LED");
    led_device.register_callback(Box::new(|params| {led_cb(params, led_driver)}));    
    
    let mut node = Node::new(rmaker.get_node_id(), "2019-02-27".to_string(), Info{}, vec![],);
    node.add_device(light_device);
    node.add_device(led_device);
    rmaker.register_node(node);
    rmaker.init_wifi()?;
    rmaker.start()?;
    rainmaker::prevent_drop();
    
    Ok(())
}
