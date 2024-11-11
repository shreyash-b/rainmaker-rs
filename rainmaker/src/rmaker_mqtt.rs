use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, LazyLock, OnceLock, RwLock},
};

use components::{
    mqtt::{MqttClient, MqttConfiguration, MqttEvent, ReceivedMessage, TLSconfiguration},
    persistent_storage::{Nvs, NvsPartition},
};

use crate::{error::RMakerError, utils::wrap_in_arc_mutex, WrappedInArcMutex};

pub(crate) trait TopicCb = Fn(ReceivedMessage) + Sync + Send + 'static;
static MQTT_INNER: OnceLock<WrappedInArcMutex<MqttClient>> = OnceLock::new();
static MQTT_CBS: LazyLock<RwLock<HashMap<String, Box<dyn TopicCb>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static PUBLISH_QUEUE: LazyLock<RwLock<HashMap<String, Vec<u8>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new())); // topic -> payload
static CONNECTED: AtomicBool = AtomicBool::new(false);

pub(crate) fn init_rmaker_mqtt() -> Result<(), RMakerError> {
    // return error if mqtt is already initialized
    if is_mqtt_initialized() {
        return Err(RMakerError("MQTT Already Initialized!".to_string()));
    }

    let fctry_partition = NvsPartition::new("fctry").unwrap();
    let fctry_nvs = Nvs::new(fctry_partition, "rmaker_creds").unwrap();

    let node_id = &crate::NODEID;
    let mut buff = vec![0; 2500];
    let mut client_cert = fctry_nvs
        .get_bytes("client_cert", &mut buff)
        .unwrap()
        .expect("Client Certificate not found in factory partition");
    let mut private_key = fctry_nvs
        .get_bytes("client_key", &mut buff)
        .unwrap()
        .expect("Client Key not found in factory partition");
    let mut server_cert = Vec::from(include_bytes!("../server_certs/rmaker_mqtt_server.crt"));

    client_cert.push(0);
    private_key.push(0);
    server_cert.push(0);

    let mqtt_tls_config = TLSconfiguration {
        // temporary workaround
        client_cert: Box::leak(Box::new(client_cert)),
        private_key: Box::leak(Box::new(private_key)),
        server_cert: Box::leak(Box::new(server_cert)),
    };

    connect(
        &MqttConfiguration {
            host: "a1p72mufdu6064-ats.iot.us-east-1.amazonaws.com",
            // host: "127.0.0.1",
            clientid: node_id.as_str(),
            port: 8883,
            // port: 1883,
        },
        Box::leak(Box::new(mqtt_tls_config)),
    )?;

    // self.mqtt_client = Some(Arc::new(Mutex::new(mqtt_client)));
    Ok(())
}

pub(crate) fn is_mqtt_initialized() -> bool {
    MQTT_INNER.get().is_some()
}

// this function is not used right now but may be required in future
#[allow(dead_code)]
pub(crate) fn is_mqtt_connected() -> bool {
    CONNECTED.load(std::sync::atomic::Ordering::SeqCst)
}

fn mqtt_callback(event: MqttEvent) {
    match event {
        MqttEvent::Received(msg) => {
            let topic = &msg.topic;
            let topic_cbs = MQTT_CBS.read().unwrap();
            if let Some(callback) = topic_cbs.get(topic) {
                callback(msg)
            }
        }

        MqttEvent::Connected => {
            CONNECTED.store(true, std::sync::atomic::Ordering::SeqCst);
            let mut mqtt = MQTT_INNER.get().unwrap().lock().unwrap();
            for topic in MQTT_CBS.read().unwrap().keys() {
                if mqtt
                    .subscribe(topic, &components::mqtt::QoSLevel::AtLeastOnce)
                    .is_err()
                {
                    log::error!("could not subscribe to {}", topic)
                };
            }
            for (topic, payload) in PUBLISH_QUEUE.read().unwrap().iter() {
                mqtt.publish(
                    topic,
                    &components::mqtt::QoSLevel::AtLeastOnce,
                    payload.to_vec(),
                );
            }
        }

        MqttEvent::Disconnected => {
            CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
        }

        _ => {}
    }
}

pub(crate) fn connect(
    config: &MqttConfiguration,
    tls_config: &'static TLSconfiguration,
) -> Result<(), RMakerError> {
    if is_mqtt_initialized() {
        Err(RMakerError("MQTT Already Initialized!".to_string()))
    } else {
        let mqtt_client = MqttClient::new(config, tls_config, Box::new(mqtt_callback))?;
        match MQTT_INNER.set(wrap_in_arc_mutex(mqtt_client)) {
            Ok(_) => Ok(()),
            Err(_) => Err(RMakerError("Could not initialized MQTT!".to_string())),
        }
    }
}

pub(crate) fn publish(topic: &str, payload: Vec<u8>) -> Result<(), RMakerError> {
    match MQTT_INNER.get() {
        Some(client) => {
            if CONNECTED.load(std::sync::atomic::Ordering::SeqCst) {
                client.lock().unwrap().publish(
                    topic,
                    &components::mqtt::QoSLevel::AtLeastOnce,
                    payload,
                );
            } else {
                // mqtt is not connected. store to publish when connected
                log::info!("mqtt not connected. queueing message");
                PUBLISH_QUEUE
                    .write()
                    .unwrap()
                    .insert(topic.to_owned(), payload);
            }
        }
        None => {
            return Err(RMakerError("MQTT Not Initialized".to_string()));
        }
    };

    Ok(())
}

pub(crate) fn subscribe(topic: &str, cb: impl TopicCb) -> Result<(), RMakerError> {
    match MQTT_INNER.get() {
        Some(client) => {
            if CONNECTED.load(std::sync::atomic::Ordering::SeqCst) {
                client
                    .lock()
                    .unwrap()
                    .subscribe(topic, &components::mqtt::QoSLevel::AtLeastOnce)?;
            }

            MQTT_CBS
                .write()
                .unwrap()
                .insert(topic.to_owned(), Box::new(cb));
        }
        None => {
            return Err(RMakerError("MQTT Not Initialized".to_string()));
        }
    };

    Ok(())
}
