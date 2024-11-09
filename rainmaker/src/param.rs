use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    name: String,
    data_type: String,
    properties: Vec<String>,
    #[serde(rename = "type")]
    param_type: ParamTypes,
    ui_type: ParamUi,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<ParamBounds>,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ParamUi {
    #[serde(rename = "esp.ui.toggle")]
    Toggle,
    #[serde(rename = "esp.ui.slider")]
    Slider,
    #[serde(rename = "esp.ui.hue-slider")]
    HueSlider,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParamBounds {
    min: i32,
    max: i32,
    step: i32,
}

impl Param {
    pub fn new(
        name: &str,
        data_type: &str,
        initial_state: Value,
        param_type: ParamTypes,
        ui_type: ParamUi,
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_initial_value(&self) -> &Value {
        &self.initial_state
    }

    pub fn add_bounds(&mut self, min: i32, max: i32, step: i32) {
        self.bounds = Some(ParamBounds { min, max, step })
    }

    pub fn new_power(name: &str, initial_state: bool) -> Self {
        Self::new(
            name,
            "bool",
            Value::Bool(initial_state),
            ParamTypes::Power,
            ParamUi::Toggle,
            vec!["read".to_string(), "write".to_string()],
        )
    }

    pub fn new_brighness(name: &str, initial_state: u32) -> Self {
        let mut param = Self::new(
            name,
            "int",
            Value::Number(initial_state.into()),
            ParamTypes::Brightness,
            ParamUi::Slider,
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
            ParamUi::HueSlider,
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
            ParamUi::Slider,
            vec!["read".to_string(), "write".to_string()],
        );

        param.add_bounds(0, 100, 1);

        param
    }
}
