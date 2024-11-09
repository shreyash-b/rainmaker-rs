/*
Node Id (node_id, String)
Config Version (config_version, String)
Information (info, Object)
    Name (name, String)
    FW Version (fw_version, String)
    Type (type, String)
Node Attributes (attributes, Array of Objects)
    Name (name, String)
    Value (value, String)
Devices (devices, Array of objects)
    Name (name, String)
    Type (type, String)
    Primary (primary, String)
    Device Attributes (attributes, Array of Objects)
        Name (name, String)
        Value (value, String)
    Params (params, Array of objects)
        Name (name, String)
        Data Type (data_type, String)
        Type (type, String)
        Properties (properties, Array of Strings)
        UI Type (ui_type, String)
        Bounds (bounds, Object)
            Minimum (min, Number)
            Maximum (max, Number)
            Step (step, Number)
*/

use std::{collections::HashMap, fmt::Debug};

use serde::Serialize;
use serde_json::Value;

use crate::device::Device;

#[derive(Debug, Clone, Serialize)]
pub struct Info {
    pub name: String,
    pub fw_version: String,
}

#[derive(Debug, Serialize)]
pub struct Node {
    node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<Info>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    attributes: HashMap<String, String>,
    devices: Vec<Device>,
}

impl Node {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            info: None,
            attributes: HashMap::new(),
            devices: Vec::new(),
        }
    }

    pub fn set_info(&mut self, info: Info) {
        self.info = Some(info);
    }

    pub fn set_attribute(&mut self, name: String, value: String) {
        self.attributes
            .insert(name, value)
            .expect("Failed to set atttribute");
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn get_param_values(&self) -> HashMap<&str, HashMap<&str, Value>> {
        let mut params = HashMap::<&str, HashMap<&str, Value>>::new();
        for dev in &self.devices {
            let mut curr_params = HashMap::<&str, Value>::new();
            for p in dev.params() {
                curr_params.insert(p.name(), p.value().clone().into());
            }
            params.insert(dev.name(), curr_params);
        }

        params
    }

    pub fn exeute_device_callback(&self, device_name: &str, params: HashMap<String, Value>) {
        for device in self.devices.iter() {
            // HIGHLY(x2) inefficient (but it works)
            if device.name() == device_name {
                device.execute_callback(params);
                break;
            }
        }
    }
}
