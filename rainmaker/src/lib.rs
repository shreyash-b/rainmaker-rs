include!(concat!(env!("OUT_DIR"), "/rainmaker.rs"));

pub mod error;
pub mod node;
pub mod wifi_prov;

use ::mdns::mdns::*;
use components::protocomm::transports::httpd::TransportHttpd;
use components::protocomm::*;

use components::{
    // local_ctrl::LocalCtrlConfig,
    mqtt::{self, MqttClient, MqttConfiguration, MqttEvent, TLSconfiguration},
    persistent_storage::{Nvs, NvsPartition},
    wifi::WifiMgr,
};
use error::RMakerError;
use node::Node;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use prost::Message;

#[cfg(target_os = "espidf")]
use wifi_prov::{WifiProvisioningConfig, WifiProvisioningMgr};

#[cfg(target_os = "espidf")]
use components::wifi::WifiClientConfig;

#[cfg(target_os = "linux")]
use std::{env, fs, path::Path};

pub type WrappedInArcMutex<T> = Arc<Mutex<T>>;

#[allow(dead_code)]
pub struct Rainmaker<'a> {
    node_id: String,
    wifi_driv: WrappedInArcMutex<WifiMgr<'a>>,
    prov_mgr: Option<wifi_prov::WifiProvisioningMgr<'a>>,
    // remove this later when mqtt client passing works for user_cloud_mapping on esp
    mqtt_client: Option<WrappedInArcMutex<MqttClient<'a>>>,
    node: Option<Arc<node::Node<'a>>>,
    local_ctrl: Option<LocalCtrlConfig<'a>>,
}

unsafe impl Send for Rainmaker<'_> {}

impl<'a> Rainmaker<'a>
where
    'a: 'static,
{
    pub fn new() -> Result<Self, RMakerError> {
        #[cfg(target_os = "linux")]
        Rainmaker::linux_init_claimdata();

        let wifi_driv = WifiMgr::new()?;

        let fctry_partition = NvsPartition::new("fctry").unwrap();
        let fctry_nvs = Nvs::new(fctry_partition, "rmaker_creds").unwrap();

        let node_id = String::from_utf8(fctry_nvs.get_bytes("node_id").unwrap()).unwrap();

        Ok(Self {
            node_id,
            wifi_driv: Arc::new(Mutex::new(wifi_driv)),
            prov_mgr: None,
            // mqtt_client: Arc::new(Mutex::new(mqtt_client)),
            mqtt_client: None,
            node: None,
            local_ctrl: Some(LocalCtrlConfig {
                protocom: None,
                node: None
            }),
        })
    }

    pub fn get_node_id(&self) -> String {
        self.node_id.clone()
    }

    pub fn init(&self) {
        #[cfg(target_os = "espidf")]
        esp_idf_svc::log::EspLogger::initialize_default();

        #[cfg(target_os = "linux")]
        simple_logger::SimpleLogger::default()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap();
    }

    pub fn start(&mut self) -> Result<(), RMakerError> {
        self.mqtt_init()?;

        let curr_node = &self.node;
        let mut mqtt = match &self.mqtt_client {
            Some(client) => client.lock().unwrap(),
            None => {
                return Err(RMakerError(
                    "Unable to start rainmaker. MQTT Client not initialized".to_string(),
                ))
            }
        };
        let node_id = self.node_id.clone();
        let node_config_topic = format!("node/{}/config", node_id);
        let params_local_init_topic = format!("node/{}/params/local/init", node_id);
        let remote_param_topic = format!("node/{}/params/remote", node_id);

        match curr_node {
            Some(node) => {
                let node_config = serde_json::to_string(node.as_ref()).unwrap();
                log::info!("publishing nodeconfig");
                mqtt.publish(
                    &node_config_topic,
                    &mqtt::QoSLevel::AtLeastOnce,
                    node_config.into(),
                );

                let init_params = node.get_init_params_string();
                let init_params = serde_json::to_string(&init_params).unwrap();
                log::info!("publishing initial params: {}", init_params);
                mqtt.publish(
                    &params_local_init_topic,
                    &mqtt::QoSLevel::AtLeastOnce,
                    init_params.into(),
                );

                // while mqtt.subscribe(remote_param_topic.as_str(), &mqtt::QoSLevel::AtLeastOnce).is_err() {
                //     log::error!("Unable to subscribe. Trying again in 10 seconds");
                //     std::thread::sleep(std::time::Duration::from_secs(10));
                // }

                // temporary workaround
                thread::sleep(Duration::from_secs(1)); // wait for connection
                if mqtt
                    .subscribe(&remote_param_topic, &mqtt::QoSLevel::AtLeastOnce)
                    .is_err()
                {
                    log::warn!(
                        "Subscribing MQTT failed. Please provision wifi and restart the node"
                    )
                };
            }
            None => panic!("error while starting: node not registered"),
        }

        // #[cfg(target_os="espidf")]

        Ok(())
    }

    pub fn report_params(&self, device_name: &str, params: HashMap<String, Value>) {
        let updated_params = json!({
            device_name: params
        });

        log::info!("reporting params: {}", updated_params.to_string());
        let mqtt = self.mqtt_client.as_ref().unwrap();
        let mut mqtt = mqtt.lock().unwrap();

        let local_params_topic = format!("node/{}/params/local", self.node_id.clone());
        mqtt.publish(
            &local_params_topic,
            &mqtt::QoSLevel::AtLeastOnce,
            updated_params.to_string().into_bytes(),
        )
    }

    pub fn register_node(&mut self, node: node::Node<'a>) {
        self.node = Some(node.into());
    }

    #[cfg(target_os = "espidf")]
    pub fn init_wifi(&mut self) -> Result<(), RMakerError> {
        
        let provisioned_status = WifiProvisioningMgr::get_provisioned_creds();
        
        match provisioned_status {
            Some((ssid, password)) => {
                log::info!(
                    "wifi already provisioned. ssid={}, password={}",
                    ssid,
                    password
                );
                
                let wifi_client_config = WifiClientConfig {
                    ssid,
                    password,
                    ..Default::default()
                };
                
                let mut wifi = self.wifi_driv.lock().unwrap();
                wifi.set_client_config(wifi_client_config).unwrap();
                wifi.start().unwrap();
                wifi.assured_connect();
                drop(wifi)
            }
            None => {
                self.mqtt_init()?;
                let prov_mgr = WifiProvisioningMgr::new(None, self.wifi_driv.clone());
                log::info!("Node not provisioned previously. Starting Wi-Fi Provisioning");
                self.prov_mgr = Some(prov_mgr);
                self.start_wifi_provisioning()?;
            }
        };
        
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    pub fn init_wifi(&mut self) -> Result<(), RMakerError> {
        log::info!("Running on linux.. Skipping WiFi setup");
        Ok(())
    }

    fn mqtt_init(&mut self) -> Result<(), RMakerError> {
        if self.mqtt_client.is_some() {
            return Ok(());
        }
        let fctry_partition = NvsPartition::new("fctry").unwrap();
        let fctry_nvs = Nvs::new(fctry_partition, "rmaker_creds").unwrap();

        let node_id = self.node_id.clone();
        let mut client_cert = fctry_nvs.get_bytes("client_cert").unwrap();
        let mut private_key = fctry_nvs.get_bytes("client_key").unwrap();
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

        let node = match &self.node {
            Some(node) => node.clone(),
            None => {
                return Err(RMakerError(
                    "Unable to intialize MQTT Client: Node not Registered".to_string(),
                ))
            }
        };

        let mqtt_client = MqttClient::new(
            &MqttConfiguration {
                host: "a1p72mufdu6064-ats.iot.us-east-1.amazonaws.com",
                // host: "127.0.0.1",
                clientid: node_id.clone().as_str(),
                port: 8883,
                // port: 1883,
            },
            Box::leak(Box::new(mqtt_tls_config)),
            Box::new(move |event| {
                mqtt_callback(event, node.to_owned());
            }),
        )
        .unwrap();

        self.mqtt_client = Some(Arc::new(Mutex::new(mqtt_client)));
        Ok(())
    }

    #[cfg(target_os = "espidf")]
    fn start_wifi_provisioning(&mut self) -> Result<(), RMakerError> {
        let prov_mgr = self.prov_mgr.as_mut().unwrap();
        prov_mgr.init(WifiProvisioningConfig {
            device_name: "ABC12".to_string(),
            scheme: wifi_prov::WifiProvScheme::SoftAP,
        });

        // while we figure out the issue about static lifetime on esp
        // #[cfg(target_os="linux")]
        let mqtt_client = match &self.mqtt_client {
            Some(client) => Some(client.clone()),
            None => {
                return Err(RMakerError(
                    "Unable to start WiFi Provisioning: MQTT Client not initialized".to_string(),
                ))
            }
        };

        // #[cfg(target_os="espidf")]
        // let mqtt_client = None;

        let node_id = self.node_id.clone();

        prov_mgr.add_endpoint("cloud_user_assoc", move |ep_name, data| -> Vec<u8> {
            cloud_user_assoc_callback(ep_name, data, node_id.to_owned(), mqtt_client.to_owned())
        });

        prov_mgr.start().unwrap();
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn linux_init_claimdata() {
        let fctry_partition = NvsPartition::new("fctry").unwrap();
        let mut rmaker_namespace = Nvs::new(fctry_partition, "rmaker_creds").unwrap();

        let node_id = rmaker_namespace.get_bytes("node_id");
        let client_cert = rmaker_namespace.get_bytes("client_cert");
        let client_key = rmaker_namespace.get_bytes("client_key");

        if node_id == None || client_cert == None || client_key == None {
            let claimdata_notfound_error = "Please set RMAKER_CLAIMDATA_LOC env variable pointing to your rainmaker claimdata folder";

            let claimdata_loc = env::var("RMAKER_CLAIMDATA_PATH").expect(claimdata_notfound_error);
            let claimdata_path = Path::new(claimdata_loc.as_str());

            if !claimdata_path.exists() {
                panic!("Claimdata folder doesn't exist");
            }

            let node_id = claimdata_path.join("node.info");
            let client_cert = claimdata_path.join("node.crt");
            let client_key = claimdata_path.join("node.key");

            if !node_id.exists() || !client_cert.exists() || !client_key.exists() {
                panic!("Claimdata folder doesn't contain valid data");
            }

            rmaker_namespace
                .set_bytes("node_id", fs::read_to_string(node_id).unwrap().as_bytes())
                .unwrap();
            rmaker_namespace
                .set_bytes(
                    "client_cert",
                    fs::read_to_string(client_cert).unwrap().as_bytes(),
                )
                .unwrap();
            rmaker_namespace
                .set_bytes(
                    "client_key",
                    fs::read_to_string(client_key).unwrap().as_bytes(),
                )
                .unwrap();
        }
    }

    pub fn local_ctrl_init(&mut self) -> Result<(), RMakerError> {

        let node_id = self.get_node_id();

        let mut mdns = MdnsService::mdns_init().unwrap();
        mdns.mdns_hostname_set(&node_id);
    
        mdns.mdns_service_add(
            &node_id, 
            "esp_local_ctrl", 
            "tcp", 
            &[
                ("node_id", &node_id), 
                ("version_endpoint", "/esp_local_ctrl/version"), 
                ("session_endpoint", "/esp_local_ctrl/session"), 
                ("control_endpoint", "/esp_local_ctrl/control"),
            ]);    

        log::info!("node id: {}", &node_id);

        let node = match &self.node {
            Some(node) => node.clone(),
            None => {
                return Err(RMakerError(
                    "Unable to intialize MQTT Client: Node not Registered".to_string(),
                ))
            }
        };

        let protocomm_new = Protocomm::new(
            ProtocomTransport::Httpd(TransportHttpd::new_1()),
            ProtocommSecurity::Sec0,
        );

        self.local_ctrl = Some(LocalCtrlConfig {
                protocom: Some(protocomm_new),
                node: Some(node.to_owned()),
            });


        let _result = LocalCtrlConfig::local_ctrl_start(&mut self.local_ctrl.as_mut().unwrap());
        Ok(())
    }
}

fn mqtt_callback<'a>(event: MqttEvent, node: Arc<Node<'a>>) {
    let print_mqtt_event = |event_name: MqttEvent| log::info!("mqtt: {event_name:?}");

    match event {
        MqttEvent::Received(msg) => {
            // for now we can let's assume the only place we'll receive this is from params/remote
            let received_val: HashMap<String, HashMap<String, Value>> =
                serde_json::from_str(&String::from_utf8(msg.payload).unwrap()).unwrap();
            let devices = received_val.keys();
            for device in devices {
                let params = received_val.get(device).unwrap().to_owned();
                node.exeute_device_callback(&device, params);
            }
        }

        MqttEvent::Connected | MqttEvent::Disconnected => print_mqtt_event(event),

        _ => {}
    }
}
pub enum PropertyTypes {
    NodeConfig = 0,
    NodeParams,
}

pub enum PropertyFlags {
    FlagReadonly = (1 << 0)
}

pub struct LocalCtrlConfig<'a> {
    pub protocom: Option<Protocomm<'a>>,
    pub node: Option<Arc<Node<'a>>>,
}

impl<'a> LocalCtrlConfig<'a> 
where
    'a : 'static
{
    pub fn local_ctrl_start(&mut self) -> anyhow::Result<(), anyhow::Error> {
        
        let pc = self.protocom.as_ref().unwrap();
        let node = self.node.as_ref().unwrap().clone();
        log::info!("adding local_ctrl listeners");

        pc.set_security_endpoint("esp_local_ctrl/session").unwrap();

        pc.register_endpoint("esp_local_ctrl/control", move |ep, data| -> Vec<u8> {
            control_handler(ep, data, node.to_owned())
        })
            .unwrap();

        pc.register_endpoint("esp_local_ctrl/version", version_handler)
            .unwrap();

        pc.start();

        Ok(())
    }
}

pub fn version_handler(
    _ep: String,
    data: Vec<u8>
) -> Vec<u8> {

    let req_proto = LocalCtrlMessage::decode(&*data).unwrap();

    log::info!("local_ctrl_version_payload: {:?}", req_proto);

    "version url Local control version v1.0".as_bytes().to_vec()
}

pub fn control_handler(
    _ep: String,
    data: Vec<u8>,
    node: Arc<Node<'_>>
) -> Vec<u8> {

    let req_proto = LocalCtrlMessage::decode(&*data).unwrap();

    log::info!("local_ctrl_payload: {:?}", req_proto);

    match req_proto.payload.clone().unwrap() {
        local_ctrl_message::Payload::CmdGetPropCount(values) => {
            println!("values are {:?}", values);
        },
        local_ctrl_message::Payload::CmdGetPropVals(values) => {
            println!("values are {:?}", values);
        },
        local_ctrl_message::Payload::CmdSetPropVals(values) => {
            println!("values are {:?}", values);
        },
        _ => unreachable!(),
    }

    let msg_type = req_proto.msg();

    let res = match msg_type {
        LocalCtrlMsgType::TypeCmdGetPropertyCount => handle_cmd_get_property_count(),
        LocalCtrlMsgType::TypeCmdGetPropertyValues => handle_cmd_get_property_values(req_proto.payload.unwrap()),
        LocalCtrlMsgType::TypeCmdSetPropertyValues => handle_cmd_set_property_values(req_proto.payload.unwrap(), node.to_owned()),
        _ => vec![]
    };

    res
}

fn handle_cmd_get_property_count() -> Vec<u8> {
    let mut resp_payload = RespGetPropertyCount::default();
    resp_payload.status = Status::Success.into();
    resp_payload.count = 2;

    let mut resp = LocalCtrlMessage::default();
    resp.payload = Some(local_ctrl_message::Payload::RespGetPropCount(resp_payload));
    resp.encode_to_vec()
}

fn handle_cmd_get_property_values(req_payload: local_ctrl_message::Payload) -> Vec<u8> {
    let mut resp_payload = RespGetPropertyValues::default();

    match req_payload {
        local_ctrl_message::Payload::CmdGetPropVals(values) => {
            resp_payload.status = Status::Success.into();

            log::info!("{:?}", values.indices);
            for i in values.indices {
                let mut prop_info = PropertyInfo::default();
                prop_info.name = "Power".to_string();
                prop_info.r#type = 2;
                prop_info.flags = 0;
                prop_info.value = vec![0];
                log::info!("Get Property {} : {:?}", i, prop_info);
                resp_payload.props.push(prop_info);
            }

            let mut resp = LocalCtrlMessage::default();
            resp.payload = Some(local_ctrl_message::Payload::RespGetPropVals(resp_payload));
            resp.encode_to_vec()
        },
        _ => unreachable!()
    }
    
}

fn handle_cmd_set_property_values(req_payload: local_ctrl_message::Payload, node: Arc<Node<'_>>) -> Vec<u8> {
    let mut resp_payload = RespSetPropertyValues::default();

    match req_payload {
        local_ctrl_message::Payload::CmdSetPropVals(values) => {
            resp_payload.status = Status::Success.into();

            log::info!("{:?}", values);
            log::info!("{:?}", std::str::from_utf8(&values.props[0].value).unwrap());

            let msg = values.props[0].value.clone();

            let received_val: HashMap<String, HashMap<String, Value>> =
                serde_json::from_str(&String::from_utf8(msg).unwrap()).unwrap();
            let devices = received_val.keys();
            for device in devices {
                let params = received_val.get(device).unwrap().to_owned();
                node.exeute_device_callback(&device, params);
            }

            let mut resp = LocalCtrlMessage::default();
            resp.payload = Some(local_ctrl_message::Payload::RespSetPropVals(resp_payload));
            resp.encode_to_vec()
        }
        _ => unreachable!() 
    }
}

pub fn cloud_user_assoc_callback<'a>(
    _ep: String,
    data: Vec<u8>,
    node_id: String,
    mqtt_client: Option<WrappedInArcMutex<MqttClient<'a>>>,
) -> Vec<u8> {
    let req_proto = RMakerConfigPayload::decode(&*data).unwrap();
    let req_payload = req_proto.payload.unwrap();

    let (user_id, secret_key) = match req_payload {
        r_maker_config_payload::Payload::CmdSetUserMapping(p) => (p.user_id, p.secret_key),
        _ => unreachable!(),
    };

    log::info!("received user_id={}, secret_key={}", user_id, secret_key);

    match mqtt_client {
        Some(mqtt) => {
            let mut mqtt_client = mqtt.lock().unwrap();

            let user_mapping_json = json!({
                "node_id": node_id,
                "user_id": user_id,
                "secret_key": secret_key,
                "reset": true
            });

            let user_mapping_topic = format!("node/{}/user/mapping", node_id);
            mqtt_client.publish(
                user_mapping_topic.as_str(),
                &mqtt::QoSLevel::AtLeastOnce,
                user_mapping_json.to_string().as_bytes().to_vec(),
            );
        }
        None => {}
    }

    let mut res_proto = RMakerConfigPayload::default();
    res_proto.msg = RMakerConfigMsgType::TypeRespSetUserMapping.into();
    res_proto.payload = Some(r_maker_config_payload::Payload::RespSetUserMapping(
        RespSetUserMapping {
            status: RMakerConfigStatus::Success.into(),
            node_id,
        },
    ));

    let res = res_proto.encode_to_vec();
    res
}

pub fn prevent_drop() {
    // eat 5-star, do nothing
    // to avoid variables from dropping
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
