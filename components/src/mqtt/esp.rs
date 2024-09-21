#![cfg(target_os = "espidf")]

use esp_idf_svc::sys::EspError;

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

impl From<esp_idf_svc::mqtt::client::EventPayload<'_, EspError>> for MqttEvent {
    fn from(value: esp_idf_svc::mqtt::client::EventPayload<'_, EspError>) -> Self {
        match value {
            esp_idf_svc::mqtt::client::EventPayload::Connected(_) => MqttEvent::Connected,
            esp_idf_svc::mqtt::client::EventPayload::Received {
                id: _,
                topic,
                data,
                details: _,
            } => {
                MqttEvent::Received(ReceivedMessage {
                    topic: topic.unwrap().to_string(), // not able to think of a case where this will be null
                    payload: data.to_vec(),
                })
            }
            esp_idf_svc::mqtt::client::EventPayload::Disconnected => MqttEvent::Disconnected,
            esp_idf_svc::mqtt::client::EventPayload::BeforeConnect => MqttEvent::BeforeConnect,
            esp_idf_svc::mqtt::client::EventPayload::Published(_) => MqttEvent::Published,
            esp_idf_svc::mqtt::client::EventPayload::Subscribed(_) => MqttEvent::Subscribed,
            _ => Self::Other,
        }
    }
}

impl<'a> MqttClient<esp_idf_svc::mqtt::client::EspMqttClient<'a>> {
    pub fn new(
        config: &MqttConfiguration,
        tls_certs: &'static TLSconfiguration,
        callback: Box<dyn Fn(MqttEvent) + Send + Sync>,
    ) -> Result<Self, Error> {
        let client_cert = std::ffi::CStr::from_bytes_with_nul(tls_certs.client_cert).unwrap();
        let private_key = std::ffi::CStr::from_bytes_with_nul(tls_certs.private_key).unwrap();
        let server_cert = std::ffi::CStr::from_bytes_with_nul(tls_certs.server_cert).unwrap();

        let options = esp_idf_svc::mqtt::client::MqttClientConfiguration {
            client_id: Some(config.clientid),
            server_certificate: Some(esp_idf_svc::tls::X509::pem(server_cert)),
            client_certificate: Some(esp_idf_svc::tls::X509::pem(client_cert)),
            private_key: Some(esp_idf_svc::tls::X509::pem(private_key)),
            ..Default::default()
        };

        let mut conn_addr = if config.port == 8883 {
            "mqtts://"
        } else {
            "mqtt://"
        }
        .to_string();

        conn_addr += config.host;

        log::info!("connection string: {}", conn_addr);

        let client = esp_idf_svc::mqtt::client::EspMqttClient::new_cb(
            conn_addr.as_str(),
            &options,
            move |req| callback(req.payload().into()),
        )
        .unwrap();

        Ok(Self { client })
    }

    pub fn publish(&mut self, topic: &str, qos: &QoSLevel, payload: Vec<u8>) -> u32 {
        let msg_id = self
            .client
            .publish(topic, qos.into(), false, payload.as_ref())
            .expect("unable to publish");

        msg_id
    }

    pub fn subscribe(&mut self, topic: &str, qos: &QoSLevel) -> Result<(), Error> {
        self.client.subscribe(topic, qos.into())?;
        Ok(())
    }
}
