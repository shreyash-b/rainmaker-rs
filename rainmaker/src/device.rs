use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::param::Param;

pub type DeviceCbType = Box<dyn Fn(HashMap<String, Value>) + Send + Sync + 'static>;

#[derive(Serialize, Deserialize)]
pub struct Device {
    name: String,
    #[serde(rename = "type")]
    device_type: DeviceType,
    #[serde(rename = "primary")]
    primary_param: Option<String>,
    attributes: HashMap<String, String>,
    params: Vec<Param>,
    #[serde(skip)]
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

    pub fn set_primary_param(&mut self, param_name: &str) {
        self.primary_param = Some(param_name.to_string())
    }

    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes
            .insert(name, value)
            .expect("Failed to add attribute");
    }

    pub fn add_param(&mut self, param: Param) {
        self.params.push(param);
    }

    pub fn register_callback(&mut self, cb: DeviceCbType) {
        self.callback = Some(Box::new(cb));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_initial_params(&self) -> HashMap<&str, &Value> {
        let mut params_value = HashMap::<&str, &Value>::new();
        for param in &self.params {
            params_value.insert(param.get_name(), param.get_initial_value());
        }

        // serde_json::to_value(params_value).unwrap().to_string()
        params_value
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

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceType {
    #[serde(rename = "esp.device.switch")]
    Switch,
    #[serde(rename = "esp.device.lightbulb")]
    Lightbulb,
    #[serde(rename = "esp.device.light")]
    Light,
    #[serde(rename = "esp.device.fan")]
    Fan,
}
