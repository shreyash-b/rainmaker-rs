pub mod base;
pub use base::*;

mod esp;
mod linux;

#[cfg(target_os = "linux")]
pub type MqttClient<'a> = base::MqttClient<rumqttc::Client>;

#[cfg(target_os = "espidf")]
pub type MqttClient<'a> = base::MqttClient<esp_idf_svc::mqtt::client::EspMqttClient<'a>>;
