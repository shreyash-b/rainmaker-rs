include!(concat!(env!("OUT_DIR"), "/rainmaker.rs"));

pub mod error;
pub mod wifi_prov;
pub mod node;

use components::{
    http::{HttpConfiguration, HttpServer}, mqtt::{self, MqttClient, MqttConfiguration, TLSconfiguration}, persistent_storage::{Nvs, NvsPartition}, wifi::{WifiClientConfig, WifiMgr}
};
use error::RMakerError;
use serde_json::json;
use std::sync::{Arc, Mutex};

use components::http::{HttpRequest, HttpResponse};
use prost::Message;
use wifi_prov::{WifiProvisioningConfig, WifiProvisioningMgr};

pub type WrappedInArcMutex<T> = Arc<Mutex<T>>;

#[cfg(target_os="linux")]
use std::{path::Path, env, fs};

pub struct Rainmaker<'a> {
    node_id: String,
    http_server: WrappedInArcMutex<HttpServer<'a>>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'a>>,
    prov_mgr: Option<wifi_prov::WifiProvisioningMgr<'a>>,
    #[allow(dead_code)] // remove this later when mqtt client passing works for user_cloud_mapping on esp
    mqtt_client: WrappedInArcMutex<MqttClient<'a>>,
    node: Option<node::Node>,
}

impl Rainmaker<'static> {
    pub fn new()  -> Result<Self, RMakerError>{
        #[cfg(target_os="linux")]
        Rainmaker::linux_init_claimdata();
        // let event_loop = EspSystemEventLoop::take().unwrap();
        let http_config = HttpConfiguration::default();
        let wifi_driv = WifiMgr::new()?;
        let http_server = HttpServer::new(&http_config).unwrap();

        let fctry_partition = NvsPartition::new("fctry").unwrap();
        let fctry_nvs = Nvs::new(fctry_partition, "rmaker_creds").unwrap();

        let node_id = String::from_utf8(fctry_nvs.get_bytes("node_id").unwrap()).unwrap();
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

        let mqtt_client = MqttClient::new(
            &MqttConfiguration {
                host: "a1p72mufdu6064-ats.iot.us-east-1.amazonaws.com",
                // host: "127.0.0.1",
                clientid: node_id.clone().as_str(),
                port: 8883,
                // port: 1883,
            },
            Box::leak(Box::new(mqtt_tls_config)),
            Box::new(|event| {
                log::info!("mqtt event: {:?}", event)
            }),
        )
        .unwrap();

        Ok(Self {
            node_id,
            http_server: Arc::new(Mutex::new(http_server)),
            wifi_driv: Arc::new(Mutex::new(wifi_driv)),
            prov_mgr: None,
            mqtt_client: Arc::new(Mutex::new(mqtt_client)),
            node: None,
        })
    }

    pub fn get_node_id_stored(&self) -> String{
        self.node_id.clone()
    }

    pub fn init(&self) {
        #[cfg(target_os = "espidf")]
        esp_idf_svc::log::EspLogger::initialize_default();

        #[cfg(target_os = "linux")]
        simple_logger::SimpleLogger::default().with_level(log::LevelFilter::Info).init().unwrap();
    }

    pub fn start(&self) -> Result<(), RMakerError> {
        let curr_node = &self.node;
        let mut mqtt = self.mqtt_client.lock().unwrap();
        let node_config_topic = format!("node/{}/config", self.node_id.clone());

        match curr_node {
            Some(n) => {
                let node_config = serde_json::to_string(n).unwrap();
                mqtt.publish(&node_config_topic, &mqtt::QoSLevel::AtLeastOnce, node_config.into())
            },
            None => panic!("error while starting: node not registered"),
        }

        Ok(())
    }

    pub fn register_node(&mut self, _node: node::Node) {
        self.node = Some(_node);
    }

    pub fn init_wifi(&mut self){
        let prov_mgr = WifiProvisioningMgr::new(
            self.http_server.clone(),
            self.wifi_driv.clone(),
        );

        
        let provisioned_status = prov_mgr.get_provisioned_creds();
        
        match provisioned_status{
            Some((ssid, password)) => {
                log::info!("wifi already provisioned. ssid={}, password={}", ssid, password);
                
                let wifi_client_config = WifiClientConfig{
                    ssid,
                    password, 
                    ..Default::default()
                };
                
                let mut wifi = self.wifi_driv.lock().unwrap();
                wifi.set_client_config(wifi_client_config).unwrap();
                wifi.start().unwrap();
                wifi.assured_connect();
                drop(wifi)
            },
            None => {
                log::info!("Node not provisioned previously. Starting Wi-Fi Provisioning");
                self.prov_mgr = Some(prov_mgr);
                self.start_wifi_provisioning();
            },
        }
    }

    fn start_wifi_provisioning(&mut self){
        let prov_mgr = self.prov_mgr.as_mut().unwrap();
        prov_mgr.init(WifiProvisioningConfig{
            device_name: "1234".to_string(),
            scheme: wifi_prov::WifiProvScheme::SoftAP
        });

        // while we figure out the issue about static lifetime on esp
        // #[cfg(target_os="linux")]
        let mqtt_client = Some(self.mqtt_client.clone());

        // #[cfg(target_os="espidf")]
        // let mqtt_client = None;

        let node_id = self.node_id.clone();

        prov_mgr.add_endpoint(
            "cloud_user_assoc".to_string(), 
            Box::new(move |req| -> HttpResponse { cloud_user_assoc_callback(req, node_id.to_owned(), mqtt_client.to_owned()) }),
        );

        prov_mgr.start().unwrap();

    }

    #[cfg(target_os="linux")]
    fn linux_init_claimdata(){
        let fctry_partition = NvsPartition::new("fctry").unwrap();
        let mut rmaker_namespace = Nvs::new(fctry_partition, "rmaker_creds").unwrap();

        
        let node_id = rmaker_namespace.get_bytes("node_id");
        let client_cert = rmaker_namespace.get_bytes("client_cert");
        let client_key = rmaker_namespace.get_bytes("client_key");
        
        if node_id == None || client_cert == None || client_key == None {
            let claimdata_notfound_error = "Please set RMAKER_CLAIMDATA_LOC env variable pointing to your rainmaker claimdata folder";

            let claimdata_loc = env::var("RMAKER_CLAIMDATA_PATH").expect(claimdata_notfound_error);
            let claimdata_path = Path::new(claimdata_loc.as_str());

            if !claimdata_path.exists(){
                panic!("Claimdata folder doesn't exist");
            }

            let node_id = claimdata_path.join("node.info");
            let client_cert = claimdata_path.join("node.crt");
            let client_key = claimdata_path.join("node.key");

            if !node_id.exists() || !client_cert.exists() || !client_key.exists() {
                panic!("Claimdata folder doesn't contain valid data");
            }

            rmaker_namespace.set_bytes("node_id", fs::read_to_string(node_id).unwrap().as_bytes()).unwrap();
            rmaker_namespace.set_bytes("client_cert", fs::read_to_string(client_cert).unwrap().as_bytes()).unwrap();
            rmaker_namespace.set_bytes("client_key", fs::read_to_string(client_key).unwrap().as_bytes()).unwrap();
        }
    }
}

pub fn cloud_user_assoc_callback<'a>(mut req: HttpRequest, node_id: String, mqtt_client: Option<WrappedInArcMutex<MqttClient<'a>>>) -> HttpResponse {
    let req_data = req.data();
    let req_proto = RMakerConfigPayload::decode(&*req_data).unwrap();
    let req_payload = req_proto.payload.unwrap();

    let (user_id, secret_key) = match req_payload {
        r_maker_config_payload::Payload::CmdSetUserMapping(p) => (p.user_id, p.secret_key),
        _ => unreachable!(),
    };

    log::info!("received user_id={}, secret_key={}", user_id, secret_key);
    
    match mqtt_client{
        Some(mqtt) => {
            let mut mqtt_client = mqtt.lock().unwrap();
            
            let user_mapping_json = json!({
                "node_id": node_id,
                "user_id": user_id,
                "secret_key": secret_key,
                "reset": true
            });
        
            let user_mapping_topic = format!("node/{}/user/mapping", node_id);
            mqtt_client.publish(user_mapping_topic.as_str(), &mqtt::QoSLevel::AtLeastOnce, user_mapping_json.to_string().as_bytes().to_vec());
        },
        None => {},
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
    HttpResponse::from_bytes(&*res)
}


pub fn prevent_drop() {
    // eat 5-star, do nothing
    // to avoid variables from dropping
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
