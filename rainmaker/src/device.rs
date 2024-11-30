//! A module for devices. Device is a physical entity which associated with a node and can be controlled from ESP-RainMaker Dashboard. One or more devices can be created and registered with node using device module.
//!
//! Example for creating an instance of device:
//! ```rust
//! let device = Device::new(name:"DeviceName", device_type:DeviceType::Switch);
//! ```
//!
//! Parameter from [Param] module can be added as following:
//! ```rust
//! let power_param = Param::new_power(name:"Power", initial_value: false);
//! device.add_param(power_param);
//! device.set_primary_param(param_name: "Power");
//! ```
//!
//! A callback needed to be set for every device in order to report updated values of parameters.
//!
//! For the reporting purpose, the function [report_params] can be used.
//!
//! Example for device callback:
//! ```rust
//! fn device_callback(params: HashMap<String, Value>){
//!     /* Write code for logging the received and reported values */
//!     rainmaker::report_params(device_name: "DeviceName", params);
//! }
//! ```
//!
//! The callback created can be associated with device using [register_callback].
//! ```rust
//! device.register_callback(Box::new(device_callback));
//! ```
//!
//! [Param]: crate::param::Param
//! [register_callback]: crate::device::Device::register_callback

use std::{collections::HashMap, fmt::Debug};

use serde::Serialize;
use serde_json::Value;

use crate::param::Param;
#[allow(unused)]
use crate::report_params;

pub(crate) type DeviceCbType = Box<dyn Fn(HashMap<String, Value>) + Send + Sync + 'static>;

#[derive(Serialize)]
pub struct Device {
    name: String,
    #[serde(rename = "type")]
    device_type: DeviceType,
    #[serde(skip_serializing_if = "Option::is_none", rename = "primary")]
    primary_param: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    attributes: HashMap<String, String>,
    params: Vec<Param>,
    #[serde(skip_serializing)]
    callback: Option<DeviceCbType>,
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.name)
            .field("device_type", &self.device_type)
            .field("primary_param", &self.primary_param)
            .field("attributes", &self.attributes)
            .field("params", &self.params)
            .finish()
    }
}

impl Device {
    /// This function creates an instance of device.
    pub fn new(name: &str, device_type: DeviceType) -> Self {
        Self {
            name: name.to_owned(),
            device_type,
            primary_param: None,
            attributes: Default::default(),
            params: vec![],
            callback: None,
        }
    }

    /// A parameter can be set as a primary parameter.
    pub fn set_primary_param(&mut self, param_name: &str) {
        self.primary_param = Some(param_name.to_string())
    }

    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes
            .insert(name, value)
            .expect("Failed to add attribute");
    }

    /// This function associates a parameter with the device.
    pub fn add_param(&mut self, param: Param) {
        self.params.push(param);
    }

    /// This function associates a callback that reports updates values of parameters.
    pub fn register_callback(&mut self, cb: DeviceCbType) {
        self.callback = Some(Box::new(cb));
    }

    /// Function for assigning a name to device.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// This function associates a list of parameters to the device.
    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub(crate) fn execute_callback(&self, params: HashMap<String, /* ParamDataType */ Value>) {
        let cb = if self.callback.is_some() {
            self.callback.as_ref().unwrap()
        } else {
            return;
        };

        cb(params);
    }
}

/// ESP RainMaker provides a set of standard devices. These are provided with a UI and have special handling in clients like Alexa/Google Home.
///
/// Refer [device list](https://rainmaker.espressif.com/docs/standard-types).
#[derive(Debug, Serialize)]
pub enum DeviceType {
    #[serde(rename = "esp.Device.Switch")]
    Switch,
    #[serde(rename = "esp.Device.Lightbulb")]
    Lightbulb,
    #[serde(rename = "esp.Device.Light")]
    Light,
    #[serde(rename = "esp.Device.Fan")]
    Fan,
    #[serde(rename = "esp.Device.TemperatureSensor")]
    TemperatureSensor,
    #[serde(rename = "esp.Device.Outlet")]
    SmartPlugOutlet,
    #[serde(rename = "esp.Device.Plug")]
    Smartplug,
    #[serde(rename = "esp.Device.Socket")]
    SmartplugSocket,
    #[serde(rename = "esp.Device.Lock")]
    Smartlock,
    #[serde(rename = "esp.Device.BlindsInternal")]
    InteriorBlind,
    #[serde(rename = "esp.Device.BlindsExternal")]
    ExteriorBlind,
    #[serde(rename = "esp.Device.GarageDoor")]
    GarageDoor,
    #[serde(rename = "esp.Device.Speaker")]
    Speaker,
    #[serde(rename = "esp.Device.AirConditioner")]
    AirConditioner,
    #[serde(rename = "esp.Device.Thermostat")]
    Thermostat,
    #[serde(rename = "esp.Device.Tv")]
    TV,
    #[serde(rename = "esp.Device.Washer")]
    Washer,
    #[serde(rename = "esp.Device.ContactSensor")]
    ContactSensor,
    #[serde(rename = "esp.Device.MotionSensor")]
    MotionSensor,
    #[serde(rename = "esp.Device.Doorbell")]
    Doorbell,
    #[serde(rename = "esp.Device.SecurityPanel")]
    SecurityPanel,
    #[serde(rename = "esp.Device.WaterHeater")]
    X,
    #[serde(rename = "esp.Device.Other")]
    OTHER,
}
