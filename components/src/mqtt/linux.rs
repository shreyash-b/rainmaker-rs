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

impl From<&rumqttc::Publish> for PublishMessage {
    fn from(value: &rumqttc::Publish) -> Self {
        Self {
            topic: value.topic.clone(),
            payload: value.payload.to_vec(),
        }
    }
}

impl From<&rumqttc::Event> for MqttEvent {
    fn from(value: &rumqttc::Event) -> Self {
        match value {
            rumqttc::Event::Incoming(e) => match e {
                rumqttc::Packet::ConnAck(_) => MqttEvent::Connected,
                rumqttc::Packet::Publish(m) => MqttEvent::Publish(m.into()),
                rumqttc::Packet::Disconnect => MqttEvent::Disconnected,
                rumqttc::Packet::Connect(_) => MqttEvent::BeforeConnect,
                _ => {
                    log::warn!("other incoming event: {:?}", e);
                    MqttEvent::Other
                }
            },

            rumqttc::Event::Outgoing(e) => match e {
                rumqttc::Outgoing::Subscribe(_) => MqttEvent::Received,
                _ => {
                    log::warn!("other outgoing event: {:?}", e);
                    MqttEvent::Other
                }
            }
        }
    }
}

impl MqttClient<rumqttc::Client> {
    pub fn new(
        config: &MqttConfiguration,
        tlscerts: &TLSconfiguration,
        callback: Box<dyn Fn(MqttEvent) + Send>,
    ) -> anyhow::Result<Self> {
        let mut option = rumqttc::MqttOptions::new(&*config.clientid, &*config.host, config.port);

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
            for (_i, notification) in conn.iter().enumerate() {
                match notification {
                    Ok(notif) => callback(MqttEvent::from(&notif)),
                    Err(e) => panic!("error while executing callback: {:?}", e),
                };
            }
        });

        Ok(Self { client })
    }

    pub fn publish(&mut self, topic: &str, qos: &QoSLevel, payload: Vec<u8>) {
        self.client
            .publish(topic, qos.into(), false, payload)
            .expect("unable to publish");
    }

    pub fn subscribe(&mut self, topic: &str, qos: &QoSLevel) {
        self.client
            .subscribe(topic, qos.into())
            .expect("unable to subscribe");
    }
}
