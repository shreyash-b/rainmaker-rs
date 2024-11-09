use serde::Serialize;
use serde_json::{Number, Value};
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct Param {
    name: String,
    param_type: ParamTypes,
    ui_type: ParamUi,
    properties: HashSet<ParamProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<ParamBounds>,
    #[serde(rename = "data_type")]
    value: ParamValue,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParamProperty {
    Read,
    Write,
}

#[derive(Debug, Clone)]
pub enum ParamValue {
    String(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub enum ParamUi {
    #[serde(rename = "esp.ui.toggle")]
    Toggle,
    #[serde(rename = "esp.ui.slider")]
    Slider,
    #[serde(rename = "esp.ui.hue-slider")]
    HueSlider,
}

#[derive(Debug, Serialize)]
struct ParamBounds {
    min: i32,
    max: i32,
    step: i32,
}

impl Param {
    pub fn new(
        name: &str,
        initial_state: ParamValue,
        param_type: ParamTypes,
        properties: HashSet<ParamProperty>,
        ui_type: ParamUi,
    ) -> Param {
        Param {
            name: name.to_owned(),
            value: initial_state,
            param_type,
            properties,
            ui_type,
            bounds: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &ParamValue {
        &self.value
    }

    pub fn add_bounds(&mut self, min: i32, max: i32, step: i32) {
        self.bounds = Some(ParamBounds { min, max, step })
    }

    pub fn new_power(name: &str, initial_value: bool) -> Self {
        let mut param_properties = HashSet::new();
        param_properties.insert(ParamProperty::Read);
        param_properties.insert(ParamProperty::Write);

        Self::new(
            name,
            ParamValue::Bool(initial_value),
            ParamTypes::Power,
            param_properties,
            ParamUi::Toggle,
        )
    }

    pub fn new_brighness(name: &str, initial_value: u32) -> Self {
        let mut param_properties = HashSet::new();
        param_properties.insert(ParamProperty::Read);
        param_properties.insert(ParamProperty::Write);

        let mut param = Self::new(
            name,
            ParamValue::Integer(initial_value as i64),
            ParamTypes::Brightness,
            param_properties,
            ParamUi::Slider,
        );
        param.add_bounds(0, 100, 1);

        param
    }

    pub fn new_hue(name: &str, initial_value: u32) -> Self {
        let mut param_properties = HashSet::new();
        param_properties.insert(ParamProperty::Read);
        param_properties.insert(ParamProperty::Write);

        let mut param = Self::new(
            name,
            ParamValue::Integer(initial_value as i64),
            ParamTypes::Hue,
            param_properties,
            ParamUi::HueSlider,
        );
        param.add_bounds(0, 360, 1);

        param
    }

    pub fn new_satuation(name: &str, initial_value: u32) -> Self {
        let mut param_properties = HashSet::new();
        param_properties.insert(ParamProperty::Read);
        param_properties.insert(ParamProperty::Write);

        let mut param = Self::new(
            name,
            ParamValue::Integer(initial_value as i64),
            ParamTypes::Saturation,
            param_properties,
            ParamUi::Slider,
        );

        param.add_bounds(0, 100, 1);

        param
    }
}

impl Serialize for ParamValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            ParamValue::String(_) => "string",
            ParamValue::Bool(_) => "bool",
            ParamValue::Integer(_) => "int",
            ParamValue::Float(_) => "float",
        })
    }
}

impl From<ParamValue> for Value {
    fn from(value: ParamValue) -> Self {
        match value {
            ParamValue::String(v) => Self::String(v),
            ParamValue::Bool(v) => Self::Bool(v),
            ParamValue::Integer(v) => Self::Number(Number::from(v)),
            ParamValue::Float(v) => Self::Number(Number::from_f64(v).unwrap()),
        }
    }
}
