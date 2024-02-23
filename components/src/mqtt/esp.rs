#![cfg(target_os = "espidf")]

use crate::{error::Error, mqtt::base::*};

impl From<&QoSLevel> for esp_idf_svc::mqtt::client::QoS {
    fn from(value: &QoSLevel) -> Self {
        match value {
            QoSLevel::AtMostOnce => esp_idf_svc::mqtt::client::QoS::AtMostOnce,
            QoSLevel::AtLeastOnce => esp_idf_svc::mqtt::client::QoS::AtLeastOnce,
            QoSLevel::ExactlyOnce => esp_idf_svc::mqtt::client::QoS::ExactlyOnce,
        }
    }
}

impl From<&esp_idf_svc::mqtt::client::Event<esp_idf_svc::mqtt::client::EspMqttMessage<'_>>>
    for MqttEvent
{
    fn from(
        value: &esp_idf_svc::mqtt::client::Event<esp_idf_svc::mqtt::client::EspMqttMessage<'_>>,
    ) -> Self {
        match value {
            esp_idf_svc::mqtt::client::Event::Connected(_) => MqttEvent::Connected,
            esp_idf_svc::mqtt::client::Event::Received(m) => MqttEvent::Received(ReceivedMessage {
                topic: m.topic().unwrap().to_string(),
                payload: Vec::from(m.data()),
            }),
            esp_idf_svc::mqtt::client::Event::Disconnected => MqttEvent::Disconnected,
            esp_idf_svc::mqtt::client::Event::BeforeConnect => MqttEvent::BeforeConnect,
            esp_idf_svc::mqtt::client::Event::Published(_) => MqttEvent::Published,
            esp_idf_svc::mqtt::client::Event::Subscribed(_) => MqttEvent::Subscribed,
            _ => Self::Other,
        }
    }
}

impl<'a> MqttClient<esp_idf_svc::mqtt::client::EspMqttClient<'a>> {
    pub fn new(
        config: &MqttConfiguration,
        tls_certs: &'static TLSconfiguration,
        callback: Box<dyn Fn(MqttEvent) + Send + Sync>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {

        let client_cert = std::ffi::CStr::from_bytes_with_nul(&tls_certs.client_cert).unwrap();
        let private_key = std::ffi::CStr::from_bytes_with_nul(&tls_certs.private_key).unwrap();
        let server_cert = std::ffi::CStr::from_bytes_with_nul(&tls_certs.server_cert).unwrap(); 
       
        let mut options = esp_idf_svc::mqtt::client::MqttClientConfiguration::default();

        options.client_id = Some(config.clientid);

        options.server_certificate = Some(esp_idf_svc::tls::X509::pem(server_cert));
        options.client_certificate = Some(esp_idf_svc::tls::X509::pem(client_cert));
        options.private_key = Some(esp_idf_svc::tls::X509::pem(private_key));

        let mut conn_addr = if config.port == 8883 {
            "mqtts://"
        } else {
            "mqtt://"
        }
        .to_string();

        conn_addr += config.host;

        log::info!("connection string: {}", conn_addr);

        let client = esp_idf_svc::mqtt::client::EspMqttClient::new(
            conn_addr.as_str(),
            &options,
            move |req| match req {
                Ok(req) => callback(req.into()),
                Err(e) => log::error!("{:?}", e),
            },
        )
        .unwrap();

        Ok(Self { client })
    }

    pub fn publish(&mut self, topic: &str, qos: &QoSLevel, payload: Vec<u8>) {
        self.client
            .publish(topic, qos.into(), false, payload.as_ref())
            .expect("unable to publish");
    }

    pub fn subscribe(&mut self, topic: &str, qos: &QoSLevel) -> Result<(), Error> {
        self.client
            .subscribe(topic, qos.into())?;
        Ok(())
    }
}
