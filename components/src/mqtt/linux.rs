#![cfg(target_os = "linux")]

use crate::error::Error;
use crate::mqtt::base::*;

impl From<&rumqttc::QoS> for QoSLevel {
    fn from(input: &rumqttc::QoS) -> Self {
        match input {
            rumqttc::QoS::AtMostOnce => QoSLevel::AtMostOnce,
            rumqttc::QoS::AtLeastOnce => QoSLevel::AtLeastOnce,
            rumqttc::QoS::ExactlyOnce => QoSLevel::ExactlyOnce,
        }
    }
}

impl From<&QoSLevel> for rumqttc::QoS {
    fn from(value: &QoSLevel) -> Self {
        match value {
            QoSLevel::AtMostOnce => rumqttc::QoS::AtMostOnce,
            QoSLevel::AtLeastOnce => rumqttc::QoS::AtLeastOnce,
            QoSLevel::ExactlyOnce => rumqttc::QoS::ExactlyOnce,
        }
    }
}

impl From<rumqttc::Publish> for ReceivedMessage {
    fn from(value: rumqttc::Publish) -> Self {
        Self {
            topic: value.topic,
            payload: value.payload.to_vec(),
        }
    }
}

impl From<rumqttc::Event> for MqttEvent {
    fn from(value: rumqttc::Event) -> Self {
        match value {
            rumqttc::Event::Incoming(e) => match e {
                rumqttc::Packet::ConnAck(_) => MqttEvent::Connected,
                rumqttc::Packet::Publish(m) => MqttEvent::Received(m.into()),
                rumqttc::Packet::Disconnect => MqttEvent::Disconnected,
                rumqttc::Packet::Connect(_) => MqttEvent::BeforeConnect,
                rumqttc::Packet::SubAck(_) => Self::Subscribed,
                rumqttc::Packet::PubAck(_) => Self::Published,
                _ => MqttEvent::Other,
            },

            rumqttc::Event::Outgoing(_) => Self::Other,
        }
    }
}

impl MqttClient<rumqttc::Client> {
    pub fn new(
        config: &MqttConfiguration,
        tlscerts: &'static TLSconfiguration,
        callback: Box<dyn Fn(MqttEvent) + Send>,
    ) -> Result<Self, Error> {
        let mut option = rumqttc::MqttOptions::new(config.clientid, config.host, config.port);
        option.transport();

        option.set_keep_alive(std::time::Duration::from_secs(60));

        option.set_transport(rumqttc::Transport::tls(
            tlscerts.server_cert.to_vec(),
            Some((
                tlscerts.client_cert.to_vec(),
                rumqttc::Key::RSA(tlscerts.private_key.to_vec()),
            )),
            None,
        ));
        let (client, mut conn) = rumqttc::Client::new(option, 5);
        std::thread::spawn(move || {
            for notification in conn.iter() {
                match notification {
                    Ok(notif) => callback(notif.into()),
                    Err(e) => log::error!("error while executing callback: {:?}", e),
                };
            }
        });

        Ok(Self { client })
    }

    pub fn publish(&mut self, topic: &str, qos: &QoSLevel, payload: Vec<u8>) -> u32 {
        self.client
            .publish(topic, qos.into(), false, payload)
            .expect("unable to publish");

        // return 0 to signify msg_id returning not supported
        0
    }

    pub fn subscribe(&mut self, topic: &str, qos: &QoSLevel) -> Result<(), Error> {
        self.client
            .subscribe(topic, qos.into())
            .expect("unable to subscribe");

        Ok(())
    }
}
