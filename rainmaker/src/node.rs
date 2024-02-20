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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceType {
    #[serde(rename="esp.device.switch")]
    Switch,
    #[serde(rename="esp.device.lightbulb")]
    Lightbulb,
    #[serde(rename="esp.device.light")]
    Light,
    #[serde(rename="esp.device.fan")]
    Fan,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    node_id: String,
    config_version: String,
    info: Info,
    attributes: Vec<NodeAttributes>,
    devices: Vec<Device>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {}
// pub struct Info {
//     pub name: String,
//     pub fw_version: String,
//     #[serde(rename = "type")]
//     pub node_type: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeAttributes {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    name: String,
    #[serde(rename = "type")]
    device_type: DeviceType,
    #[serde(rename = "primary")]
    primary_param: String,
    attributes: Vec<DeviceAttributes>,
    params: Vec<Param>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceAttributes {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    name: String,
    data_type: String,
    properties: Vec<String>,
    ui_type: String,
    bounds: Option<Bounds>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bounds {
    min: Option<i32>,
    max: Option<i32>,
    step: Option<i32>,
}

impl Node {
    pub fn new(
        node_id: String,
        config_version: String,
        info: Info,
        attributes: Vec<NodeAttributes>,
        device: Vec<Device>,
    ) -> Node {
        Node {
            node_id,
            config_version,
            info,
            attributes,
            devices: device,
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }
}

impl Device {
    pub fn new(
        name: &str,
        device_type: DeviceType,
        primary_param: &str,
        attributes: Vec<DeviceAttributes>,
        params: Vec<Param>,
    ) -> Device {
        Device {
            name: name.to_owned(),
            device_type,
            primary_param: primary_param.to_owned(),
            attributes,
            params,
        }
    }

    pub fn add_param(&mut self, param: Param) {
        self.params.push(param);
    }
}

impl Param {
    pub fn new(name: &str, data_type: &str, properties: Vec<String>, ui_type: &str) -> Param {
        Param {
            name: name.to_owned(),
            data_type: data_type.to_owned(),
            properties,
            ui_type: ui_type.to_owned(),
            // bounds: None, 
            bounds: Some(Bounds{
                min: Some(0),
                max: Some(100),
                step: Some(1)
            })
        }
    }
}