include!(concat!(env!("OUT_DIR"), "/rainmaker.rs"));

pub mod wifi_prov;
pub mod error;
use error::RMakerError;

use components::{
    http::{HttpConfiguration, HttpServer},
    mqtt::{self, MqttClient, MqttConfiguration, TLSconfiguration},
    wifi::{WifiClientConfig, WifiMgr},
};
use serde_json::json;
use std::sync::{Arc, Mutex};

use components::http::{HttpRequest, HttpResponse};
use prost::Message;
use wifi_prov::{WifiProvisioningConfig, WifiProvisioningMgr};

pub type WrappedInArcMutex<T> = Arc<Mutex<T>>;

pub struct Rainmaker<'a> {
    node_id: String,
    http_server: WrappedInArcMutex<HttpServer<'a>>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'a>>,
    prov_mgr: Option<wifi_prov::WifiProvisioningMgr<'a>>,
    #[allow(dead_code)] // remove this later when mqtt client passing works for user_cloud_mapping on esp
    mqtt_client: WrappedInArcMutex<MqttClient<'a>>, 
}

impl Rainmaker<'static> {
    pub fn new()  -> Result<Self, RMakerError>{
        // let event_loop = EspSystemEventLoop::take().unwrap();
        let http_config = HttpConfiguration::default();
        let wifi_driv = WifiMgr::new()?;
        let http_server = HttpServer::new(&http_config).unwrap();

        let node_id = "58CF79DAC1E4".to_string();
        let mut client_cert = Vec::from(include_bytes!("/home/shreyash/.espressif/rainmaker/claim_data/Google_6hS8QpvnNhmAmd2FKPjoyd/58CF79DAC1E4/node.crt"));
        let mut private_key = Vec::from(include_bytes!("/home/shreyash/.espressif/rainmaker/claim_data/Google_6hS8QpvnNhmAmd2FKPjoyd/58CF79DAC1E4/node.key"));
        let mut server_cert = Vec::from(include_bytes!("/home/shreyash/esp/esp-rainmaker/components/esp_rainmaker/server_certs/rmaker_mqtt_server.crt"));

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
                clientid: node_id.clone().as_str(),
                port: 8883,
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
        })
    }

    pub fn init(&self) {
        #[cfg(target_os = "espidf")]
        esp_idf_svc::log::EspLogger::initialize_default();

        #[cfg(target_os = "linux")]
        simple_logger::SimpleLogger::default().with_level(log::LevelFilter::Info).init().unwrap();
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
                log::info!("trying to connect wifi");
                
                let wifi_client_config = WifiClientConfig{
                    ssid,
                    password, 
                    ..Default::default()
                };
                
                let mut wifi = self.wifi_driv.lock().unwrap();
                wifi.set_client_config(wifi_client_config).unwrap();
                wifi.start().unwrap();
                wifi.connect().unwrap();
                drop(wifi)
            },
            None => {
                log::info!("Node not provisioned previously. Starting Wi-Fi Provisioning");
                self.prov_mgr = Some(prov_mgr);
                self.start_wifi_provisioning();
            },
        }
    }

    pub fn start_wifi_provisioning(&mut self){
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