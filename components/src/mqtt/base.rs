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

pub struct TLSconfiguration<'a> {
    pub client_cert: &'a Vec<u8>,
    pub private_key: &'a Vec<u8>,
    pub server_cert: &'a Vec<u8>,
}

#[derive(Debug)]
pub struct ReceivedMessage {
    pub topic: String,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub enum MqttEvent {
    Connected,
    Disconnected,
    Published,
    Subscribed,
    BeforeConnect,
    Received(ReceivedMessage),
    Other,
}
