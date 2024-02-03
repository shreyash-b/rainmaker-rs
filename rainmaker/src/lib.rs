pub mod wifi_prov;
use components::{wifi::WifiMgr, http::{HttpServer, HttpConfiguration}};
use std::sync::{Arc, Mutex};

use components::http::{HttpRequest, HttpResponse};
use prost::Message;
use wifi_prov::{WifiProvisioningConfig, WifiProvisioningMgr};

pub type WrappedInArcMutex<T> = Arc<Mutex<T>>;

include!(concat!(env!("OUT_DIR"), "/rainmaker.rs"));

pub struct Rainmaker<'a> {
    http_server: WrappedInArcMutex<HttpServer<'a>>,
    wifi_driv: WrappedInArcMutex<WifiMgr<'a>>,
    prov_mgr: Option<wifi_prov::WifiProvisioningMgr<'a>>,
    // event_loop: EspSystemEventLoop,
}

impl<'a> Rainmaker<'a> {
    pub fn new() -> Self {
        // let event_loop = EspSystemEventLoop::take().unwrap();
        let http_config = HttpConfiguration::default();
        let wifi_driv = WifiMgr::new();
        let http_server = HttpServer::new(&http_config).unwrap();

        Self {
            http_server: Arc::new(Mutex::new(http_server)),
            wifi_driv: Arc::new(Mutex::new(wifi_driv)),
            // event_loop,
            prov_mgr: None,
        }
    }

    pub fn init(&self) {
        #[cfg(target_os = "espidf")]
        esp_idf_svc::log::EspLogger::initialize_default();

        #[cfg(target_os = "linux")]
        simple_logger::SimpleLogger::new().init().unwrap();
    }

    pub fn init_prov(&mut self, prov_config: WifiProvisioningConfig) {
        let prov_mgr = WifiProvisioningMgr::new(
            self.http_server.clone(),
            self.wifi_driv.clone(),
            prov_config,
        );

        prov_mgr.add_endpoint(
            "cloud_user_assoc".to_string(),
            Box::new(|req| -> HttpResponse { cloud_user_assoc_callback(req) }),
        );

        self.prov_mgr = Some(prov_mgr);
    }

    pub fn start_prov(&self) {
        self.prov_mgr.as_ref().unwrap().start();
    }
}

pub fn prevent_drop(){
    // eat 5-star, do nothing
    // to avoid variables from dropping
    loop{
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

pub fn cloud_user_assoc_callback(mut req: HttpRequest) -> HttpResponse {
    let req_data = req.data();
    let req_proto = RMakerConfigPayload::decode(&*req_data).unwrap();
    let req_payload = req_proto.payload.unwrap();

    let (user_id, secret_key) = match req_payload {
        r_maker_config_payload::Payload::CmdSetUserMapping(p) => (p.user_id, p.secret_key),
        _ => unreachable!(),
    };

    log::info!("received user_id={}, secret_key={}", user_id, secret_key);

    let mut res_proto = RMakerConfigPayload::default();
    res_proto.msg = RMakerConfigMsgType::TypeRespSetUserMapping.into();
    res_proto.payload = Some(r_maker_config_payload::Payload::RespSetUserMapping(
        RespSetUserMapping {
            status: RMakerConfigStatus::Success.into(),
            node_id: "58CF79DAC1E4".to_string(),
        },
    ));
    
    let res = res_proto.encode_to_vec();
    HttpResponse::from_bytes(&*res)
}
