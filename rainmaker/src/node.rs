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

type DeviceCbType<'a> = Box<dyn Fn(HashMap<String, Value>) + Send + Sync + 'a>;
// type DeviceCbType<'a> = Box<dyn Fn(HashMap<String, ParamDataType>) + Send + Sync + 'a>;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Node<'a> {
    node_id: String,
    config_version: String,
    info: Info,
    attributes: Vec<NodeAttributes>,
    devices: Vec<Device<'a>>,
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

#[derive(Serialize, Deserialize)]
pub struct Device<'a> {
    name: String,
    #[serde(rename = "type")]
    device_type: DeviceType,
    #[serde(rename = "primary")]
    primary_param: String,
    attributes: Vec<DeviceAttributes>,
    params: Vec<Param>,
    #[serde(skip)]
    callback: Option<DeviceCbType<'a>>,
}

impl Debug for Device<'_> {
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
    #[serde(rename = "type")]
    param_type: ParamTypes,
    ui_type: UiTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<Bounds>,
    initial_state: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParamTypes {
    #[serde(rename = "esp.param.power")]
    Power,
    #[serde(rename = "esp.param.brightness")]
    Brightness,
    #[serde(rename = "esp.param.hue")]
    Hue,
    #[serde(rename = "esp.param.saturation")]
    Saturation,
}

#[derive(Debug)]
pub enum ParamDataType {
    // #[serde(rename="bool")]
    Boolean(bool),
    // #[serde(rename="int")]
    Integer(i64),
    // #[serde(rename="float")]
    Float(f64),
    None,
}

impl From<serde_json::Value> for ParamDataType {
    fn from(value: serde_json::Value) -> Self {
        match value {
            // Value::Null => todo!(),
            Value::Bool(bool) => Self::Boolean(bool),
            Value::Number(num) => {
                if num.is_i64() {
                    Self::Integer(num.as_i64().unwrap())
                } else if num.is_f64() {
                    Self::Float(num.as_f64().unwrap())
                } else {
                    Self::None
                }
            }
            // Value::String(_) => todo!(),
            // Value::Array(_) => todo!(),
            // Value::Object(_) => todo!(),
            _ => Self::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UiTypes {
    #[serde(rename = "esp.ui.toggle")]
    Toggle,
    #[serde(rename = "esp.ui.slider")]
    Slider,
    #[serde(rename = "esp.ui.hue-slider")]
    HueSlider,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bounds {
    min: i32,
    max: i32,
    step: i32,
}

impl<'a> Node<'a> {
    pub fn new(
        node_id: String,
        config_version: String,
        info: Info,
        attributes: Vec<NodeAttributes>,
    ) -> Self {
        Self {
            node_id,
            config_version,
            info,
            attributes,
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Device<'a>) {
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
            device_params.insert(&device.name, device_initial_params);
        }

        // let params_init = serde_json::to_value(device_params).unwrap();

        // params_init.to_string()
        device_params
    }
}

impl<'a> Device<'a> {
    pub fn new(
        name: &str,
        device_type: DeviceType,
        primary_param: &str,
        attributes: Vec<DeviceAttributes>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            device_type,
            primary_param: primary_param.to_owned(),
            attributes,
            params: vec![],
            callback: None,
        }
    }

    pub fn add_param(&mut self, param: Param) {
        self.params.push(param);
    }

    pub fn register_callback(&mut self, cb: DeviceCbType<'a>) {
        self.callback = Some(Box::new(cb));
    }

    pub fn execute_callback(&self, params: HashMap<String, /* ParamDataType */ Value>) {
        let cb: &DeviceCbType;
        if self.callback.is_some() {
            cb = self.callback.as_ref().unwrap();
        } else {
            return;
        }
        cb(params);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    // pub fn get_initial_params(&self) -> String{
    pub fn get_initial_params(&self) -> HashMap<&str, &Value> {
        let mut params_value = HashMap::<&str, &Value>::new();
        for param in &self.params {
            params_value.insert(&param.name, &param.initial_state);
        }

        // serde_json::to_value(params_value).unwrap().to_string()
        params_value
    }
}

impl Param {
    pub fn new(
        name: &str,
        data_type: &str,
        initial_state: Value,
        param_type: ParamTypes,
        ui_type: UiTypes,
        properties: Vec<String>,
    ) -> Param {
        Param {
            name: name.to_owned(),
            data_type: data_type.to_owned(),
            properties,
            param_type,
            ui_type,
            bounds: None,
            initial_state,
        }
    }

    pub fn add_bounds(&mut self, min: i32, max: i32, step: i32) {
        self.bounds = Some(Bounds { min, max, step })
    }

    pub fn new_power(name: &str, initial_state: bool) -> Self {
        Self::new(
            name,
            "bool",
            Value::Bool(initial_state),
            ParamTypes::Power,
            UiTypes::Toggle,
            vec!["read".to_string(), "write".to_string()],
        )
    }

    pub fn new_brighness(name: &str, initial_state: u32) -> Self {
        let mut param = Self::new(
            name,
            "int",
            Value::Number(initial_state.into()),
            ParamTypes::Brightness,
            UiTypes::Slider,
            vec!["read".to_string(), "write".to_string()],
        );
        param.add_bounds(0, 100, 1);

        param
    }

    pub fn new_hue(name: &str, initial_state: u32) -> Self {
        let mut param = Self::new(
            name,
            "int",
            Value::Number(initial_state.into()),
            ParamTypes::Hue,
            UiTypes::HueSlider,
            vec!["read".to_string(), "write".to_string()],
        );
        param.add_bounds(0, 360, 1);

        param
    }

    pub fn new_satuation(name: &str, initial_state: u32) -> Self {
        let mut param = Self::new(
            name,
            "int",
            initial_state.into(),
            ParamTypes::Saturation,
            UiTypes::Slider,
            vec!["read".to_string(), "write".to_string()],
        );

        param.add_bounds(0, 100, 1);

        param
    }
}
