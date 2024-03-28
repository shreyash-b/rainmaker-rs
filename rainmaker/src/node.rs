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
use serde_json::{from_slice, from_str, Value};
use components::persistent_storage::*;


type DeviceCbType<'a> = Box<dyn Fn(&HashMap<String, Value>) + Send + Sync + 'a>;
type ServiceCbType<'a> = Box<dyn Fn(&HashMap<String, Value>) + Send + Sync + 'a>;

// type ParamCbType<'a> = Box<dyn Fn(HashMap<String, ParamDataType>) + Send + Sync + 'a>;

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
    #[serde(rename = "esp.service.scenes")]
    Scenes,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParamDataType {
    #[serde(rename="bool")]
    Boolean,
    #[serde(rename="int")]
    Integer,
    #[serde(rename="float")]
    Float,
    #[serde(rename="string")]
    String,
    #[serde(rename="array")]
    Array,
    #[serde(rename="object")]
    Object,
    #[serde(rename="invalid")]
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node<'a> {
    node_id: String,
    config_version: String,
    info: Info,
    attributes: Vec<NodeAttributes>,
    devices: Vec<Device<'a>>,
    services: Vec<Service<'a>>,
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

#[derive(Serialize, Deserialize)]
pub struct Service<'a> {
    name: String,
    #[serde(rename = "type")]
    device_type: DeviceType,
    #[serde(rename = "primary")]
    primary_param: String,
    attributes: Vec<DeviceAttributes>,
    params: Vec<Param>,
    #[serde(skip)]
    callback: Option<ServiceCbType<'a>>,
}

impl Debug for Service<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Service")
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
    data_type: ParamDataType,
    properties: Vec<String>,
    #[serde(rename = "type")]
    param_type: ParamTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_type: Option<UiTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<Bounds>,
    initial_state: Value
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
    #[serde(rename = "esp.param.scenes")]
    Scene,
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
            services: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Device<'a>) {
        self.devices.push(device);
    }

    pub fn add_service(&mut self, service: Service<'a>) {
        self.services.push(service);
    }

    pub fn exeute_device_callback(&self, device_name: &str, params: &HashMap<String, Value>) {
        let mut found = false;
        for device in self.devices.iter() {
            // HIGHLY(x2) inefficient (but it works)
            if device.get_name() == device_name {
                device.execute_callback(params);
                found = true;
                break;
            }
        }
        if !found {
            for service in self.services.iter() {
                if service.get_name() == device_name {
                    service.execute_callback(params, &self);
                    found = true;
                    break;
                }
            }
        }
        if !found {
            log::info!("Received a request to execute callback for unknown device/service {}", device_name);
        }
    }

    pub fn get_init_params_string(&self) -> HashMap<&str, HashMap<&str, &Value>>{
        let mut device_params = HashMap::<&str, HashMap<&str, &Value>>::new();
        for device in &self.devices{
            let device_initial_params = device.get_initial_params();
            device_params.insert(&device.name, device_initial_params);
        };
        for service in &self.services {
            let service_initial_params = service.get_initial_params();
            device_params.insert(&service.name, service_initial_params);
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

    pub fn execute_callback(&self, params: &HashMap<String, /* ParamDataType */ Value>) {
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
    pub fn get_initial_params(&self) -> HashMap<&str, &Value>{
        let mut params_value = HashMap::<&str, &Value>::new();
        for param in &self.params{
            if param.properties.contains(&"persist".to_string()) {
                let mut nvs = Nvs::new(NvsPartition::new("nvs").unwrap(), &self.name).unwrap();
                if let Some(p) = nvs.get_bytes(&param.name) {
                    let json_string: String = String::from_utf8(p).expect("Failed to convert to string");
                    param.initial_state = from_str(&json_string).expect("Failed to convert to Value");
                }
                else {
                    nvs.set_bytes(&param.name, &param.initial_state.to_string().into_bytes());
                }
            }
            params_value.insert(&param.name, &param.initial_state);
        }

        // serde_json::to_value(params_value).unwrap().to_string()
        params_value
    }
}

impl<'a> Service<'a> {
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

    pub fn register_callback(&mut self, cb: ServiceCbType<'a>) {
        self.callback = Some(Box::new(cb));
    }

    pub fn execute_callback(&self, params: &HashMap<String, /* ParamDataType */ Value>, node: &Node) {
        let cb: &ServiceCbType;
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

    pub fn get_initial_params(&self) -> HashMap<&str, &Value>{
        let mut params_value = HashMap::<&str, &Value>::new();
        for param in &self.params{
            params_value.insert(&param.name, &param.initial_state);
        }

        // serde_json::to_value(params_value).unwrap().to_string()
        params_value
    }
}

impl Param {
    pub fn new(
        name: &str,
        data_type: ParamDataType,
        initial_state: Value,
        param_type: ParamTypes,
        ui_type: UiTypes,
        properties: Vec<String>,
    ) -> Param {
        Param {
            name: name.to_owned(),
            data_type,
            properties,
            param_type,
            bounds: None,
            initial_state,
            ui_type: Some(ui_type),
        }
    }

    pub fn new_without_ui(
        name: &str,
        data_type: ParamDataType,
        initial_state: Value,
        param_type: ParamTypes,
        properties: Vec<String>
    ) -> Param {
        Param {
            name: name.to_owned(),
            data_type,
            properties,
            param_type,
            bounds: None,
            initial_state,
            ui_type: None,
        }
    }

    pub fn add_bounds(&mut self, min: i32, max: i32, step: i32) {
        self.bounds = Some(Bounds { min, max, step })
    }

    pub fn new_power(name: &str, initial_state: bool) -> Self {
        Self::new(
            name,
            ParamDataType::Boolean,
            Value::Bool(initial_state),
            ParamTypes::Power,
            UiTypes::Toggle,
            vec!["read".to_string(), "write".to_string()],
        )
    }

    pub fn new_brighness(name: &str, initial_state: u32) -> Self {
        let mut param = Self::new(
            name,
            ParamDataType::Integer,
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
            ParamDataType::Integer,
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
            ParamDataType::Integer,
            initial_state.into(),
            ParamTypes::Saturation,
            UiTypes::Slider, 
            vec!["read".to_string(), "write".to_string()]
        );

        param.add_bounds(0, 100, 1);

        param
    }
}
