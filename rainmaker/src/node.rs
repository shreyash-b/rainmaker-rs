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

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::device::Device;

const CONFIG_VER: &str = "2019-02-27";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub name: String,
    pub fw_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    node_id: String,
    config_version: String,
    info: Option<NodeInfo>,
    attributes: HashMap<String, String>,
    devices: Vec<Device>,
}

impl Node {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            config_version: CONFIG_VER.to_string(),
            info: None,
            attributes: HashMap::new(),
            devices: Vec::new(),
        }
    }

    pub fn set_info(&mut self, info: NodeInfo) {
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

    pub fn exeute_device_callback(&self, device_name: &str, params: HashMap<String, Value>) {
        for device in self.devices.iter() {
            // HIGHLY(x2) inefficient (but it works)
            if device.get_name() == device_name {
                device.execute_callback(params);
                break;
            }
        }
    }

    pub fn get_init_params_string(&self) -> HashMap<&str, HashMap<&str, &Value>> {
        let mut device_params = HashMap::<&str, HashMap<&str, &Value>>::new();
        for device in &self.devices {
            let device_initial_params = device.get_initial_params();
            device_params.insert(device.get_name(), device_initial_params);
        }

        // let params_init = serde_json::to_value(device_params).unwrap();

        // params_init.to_string()
        device_params
    }
}
