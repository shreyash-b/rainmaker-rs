#![feature(trait_alias)]

pub mod device;
pub mod error;
pub mod node;
pub(crate) mod proto;
pub mod param;
pub(crate) mod utils;

mod constants;
mod rmaker_mqtt;
use components::{
    mqtt::ReceivedMessage,
    persistent_storage::{Nvs, NvsPartition},
    wifi_prov::{WiFiProvTransportTrait, WifiProvMgr},
};
use constants::*;
use error::RMakerError;
use node::Node;
use proto::esp_rmaker_user_mapping::*;
use quick_protobuf::{MessageWrite, Writer};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex, OnceLock},
    thread,
    time::Duration,
};

#[cfg(target_os = "linux")]
use std::{env, fs, path::Path};

pub type WrappedInArcMutex<T> = Arc<Mutex<T>>;

static NODEID: LazyLock<String> = LazyLock::new(|| {
    let fctry_partition = NvsPartition::new("fctry").unwrap();
    let fctry_nvs = Nvs::new(fctry_partition, "rmaker_creds").unwrap();
    let mut buff = [0; 32];
    let bytes = fctry_nvs
        .get_bytes("node_id", &mut buff)
        .unwrap()
        .expect("Node id not found in NVS");
    String::from_utf8(bytes).unwrap()
});

#[derive(Debug)]
pub struct Rainmaker {
    node: Option<Arc<node::Node>>,
}

static mut RAINMAKER: OnceLock<Rainmaker> = OnceLock::new();

impl Rainmaker {
    pub fn init() -> Result<&'static mut Self, RMakerError> {
        #[cfg(target_os = "linux")]
        Self::linux_init_claimdata();

        if unsafe { RAINMAKER.get().is_some() } {
            return Err(RMakerError("Rainmaker already initialized".to_string()));
        }
        unsafe {
            RAINMAKER.set(Self { node: None }).unwrap();
        }
        Ok(unsafe { RAINMAKER.get_mut().unwrap() })
    }

    pub fn get_node_id(&self) -> String {
        NODEID.to_string()
    }

    pub fn start(&mut self) -> Result<(), RMakerError> {
        // initialize mqtt if not done already
        if !rmaker_mqtt::is_mqtt_initialized() {
            rmaker_mqtt::init_rmaker_mqtt()?;
        }

        let curr_node = &self.node;
        let node_id = NODEID.to_string();
        let node_config_topic = format!("node/{}/{}", node_id, NODE_CONFIG_TOPIC_SUFFIX);
        let params_local_init_topic =
            format!("node/{}/{}", node_id, NODE_PARAMS_LOCAL_INIT_TOPIC_SUFFIX);
        let remote_param_topic = format!("node/{}/{}", node_id, NODE_PARAMS_REMOTE_TOPIC_SUFFIX);

        match curr_node {
            Some(node) => {
                let node_config = serde_json::to_string(node.as_ref()).unwrap();
                log::info!("publishing nodeconfig");
                rmaker_mqtt::publish(&node_config_topic, node_config.into())?;

                let init_params = node.get_init_params_string();
                let init_params = serde_json::to_string(&init_params).unwrap();
                log::info!("publishing initial params: {}", init_params);
                rmaker_mqtt::publish(&params_local_init_topic, init_params.into())?;
                let node = node.clone();
                thread::sleep(Duration::from_secs(1)); // wait for connection
                rmaker_mqtt::subscribe(&remote_param_topic, move |msg| {
                    remote_params_callback(msg, node.to_owned())
                })?
            }
            None => panic!("error while starting: node not registered"),
        }

        Ok(())
    }

    pub fn register_node(&mut self, node: Node) {
        self.node = Some(node.into());
    }

    pub fn reg_user_mapping_ep<T: WiFiProvTransportTrait>(&self, prov_mgr: &mut WifiProvMgr<T>) {
        let node_id = self.get_node_id();
        prov_mgr.add_endpoint(
            "cloud_user_assoc",
            Box::new(move |ep, data| -> Vec<u8> { cloud_user_assoc_callback(ep, data, &node_id) }),
        )
    }

    #[cfg(target_os = "linux")]
    fn linux_init_claimdata() {
        let fctry_partition = NvsPartition::new("fctry").unwrap();
        let mut rmaker_namespace = Nvs::new(fctry_partition, "rmaker_creds").unwrap();

        let mut buff = vec![0; 2500];
        let node_id = rmaker_namespace.get_bytes("node_id", &mut buff).unwrap();
        let client_cert = rmaker_namespace
            .get_bytes("client_cert", &mut buff)
            .unwrap();
        let client_key = rmaker_namespace.get_bytes("client_key", &mut buff).unwrap();

        if node_id.is_none() || client_cert.is_none() || client_key.is_none() {
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
}

fn remote_params_callback(msg: ReceivedMessage, node: Arc<Node>) {
    let received_val: HashMap<String, HashMap<String, Value>> =
        serde_json::from_str(&String::from_utf8(msg.payload).unwrap()).unwrap();
    let devices = received_val.keys();
    for device in devices {
        let params = received_val.get(device).unwrap().to_owned();
        node.exeute_device_callback(device, params);
    }
}

fn cloud_user_assoc_callback(_ep: &str, data: &[u8], node_id: &str) -> Vec<u8> {
    let req_proto = RMakerConfigPayload::try_from(data).unwrap();
    let req_payload = req_proto.payload;

    let (user_id, secret_key) = match req_payload {
        mod_RMakerConfigPayload::OneOfpayload::cmd_set_user_mapping(p) => (p.UserID, p.SecretKey),
        _ => unreachable!(),
    };

    log::info!("received user_id={}, secret_key={}", user_id, secret_key);

    let user_mapping_json = json!({
        "node_id": node_id,
        "user_id": user_id,
        "secret_key": secret_key,
        "reset": true
    });

    let user_mapping_topic = format!("node/{}/{}", node_id, USER_MAPPING_TOPIC_SUFFIX);

    if !rmaker_mqtt::is_mqtt_initialized() && rmaker_mqtt::init_rmaker_mqtt().is_err() {
        // cannot publish user mapping payload
        return vec![0];
    }

    if rmaker_mqtt::publish(
        &user_mapping_topic,
        user_mapping_json.to_string().as_bytes().to_vec(),
    )
    .is_err()
    {
        log::error!("could not publish user mapping payload");
    }

    let res_proto = RMakerConfigPayload {
        msg: RMakerConfigMsgType::TypeRespSetUserMapping,
        payload: mod_RMakerConfigPayload::OneOfpayload::resp_set_user_mapping(RespSetUserMapping {
            Status: RMakerConfigStatus::Success,
            NodeId: node_id.to_string(),
        }),
    };

    let mut out_vec = vec![];
    let mut writer = Writer::new(&mut out_vec);

    res_proto.write_message(&mut writer).unwrap();

    out_vec
}

pub fn report_params(device_name: &str, params: HashMap<String, Value>) {
    let updated_params = json!({
        device_name: params
    });

    let local_params_topic = format!("node/{}/params/local", NODEID.as_str());
    rmaker_mqtt::publish(&local_params_topic, updated_params.to_string().into_bytes()).unwrap();
}
