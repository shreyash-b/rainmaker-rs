pub enum QoSLevel {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

pub struct MqttClient<T> {
    pub(crate) client: T,
}

pub struct MqttConfiguration<'a> {
    pub host: &'a str,
    pub clientid: &'a str,
    pub port: u16,
}

pub struct PublishMessage {
    pub topic: String,
    pub payload: Vec<u8>,
}

pub enum MqttEvent {
    Connected,
    Disconnected,
    Publish(PublishMessage),
    BeforeConnect,
    Received,
    Other,
}
