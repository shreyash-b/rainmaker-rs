mod proto;
mod security;
pub mod transports;

use crate::{error::Error, http::HttpConfiguration};
pub use prost::Message;
pub use proto::*;
use std::sync::Arc;
use transports::ble::{TransportBle, TransportBleConfig};
use transports::httpd::TransportHttpd;

use transports::TransportTrait;

pub use self::security::ProtocommSecurity;
use self::security::SecurityTrait;

const LOGGER_TAG: &str = "protocomm";

pub type ProtocommCallbackType = Box<dyn Fn(&str, Vec<u8>) -> Vec<u8> + Send + Sync + 'static>;

pub enum ProtocomTransportConfig {
    Httpd(HttpConfiguration),
    Ble(TransportBleConfig),
}

pub struct ProtocommConfig {
    pub security: ProtocommSecurity,
    pub transport: ProtocomTransportConfig,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EndpointType {
    Version,
    Security,
    #[default]
    Other,
}

pub struct Protocomm {
    transport: Box<dyn TransportTrait>, // change this to accepting trait after implementing BLE transport
    sec: Arc<ProtocommSecurity>,
}

impl Protocomm {
    pub fn new(config: ProtocommConfig) -> Self {
        let transport: Box<dyn TransportTrait> = match config.transport {
            ProtocomTransportConfig::Httpd(config) => Box::new(TransportHttpd::new(config)),
            ProtocomTransportConfig::Ble(config) => Box::new(TransportBle::new(config)),
        };

        Self {
            transport,
            sec: Arc::new(config.security),
        }
    }

    pub fn start(&mut self) {
        log::info!("Starting Protocomm");
        self.transport.start();
    }

    pub fn register_endpoint(
        &mut self,
        ep_name: &str,
        callback: ProtocommCallbackType,
    ) -> Result<(), Error> {
        self.register_endpoint_internal(ep_name, callback, EndpointType::Other)
    }

    pub fn set_security_endpoint(&mut self, ep_name: &str) -> Result<(), Error> {
        // supply dummy callback here, appropriate callback is called
        self.register_endpoint_internal(
            ep_name,
            Box::new(|_ep, _data| vec![]),
            EndpointType::Security,
        )
    }

    pub fn set_version_endpoint(
        &mut self,
        ep_name: &str,
        version_str: String,
    ) -> Result<(), Error> {
        self.register_endpoint_internal(
            ep_name,
            Box::new(move |_ep, _data| version_str.to_owned().into()),
            EndpointType::Version,
        )
    }

    fn register_endpoint_internal(
        &mut self,
        ep_name: &str,
        callback: ProtocommCallbackType,
        ep_type: EndpointType,
    ) -> Result<(), Error> {
        self.transport
            .add_endpoint(ep_name, callback, ep_type, self.sec.clone());
        log::debug!(target: LOGGER_TAG, "registered handler for {ep_name}");

        Ok(())
    }
}

pub(crate) fn protocomm_req_handler(
    ep: &str,
    data: Vec<u8>,
    cb: &ProtocommCallbackType,
    ep_type: &EndpointType,
    sec: &Arc<ProtocommSecurity>,
) -> Vec<u8> {
    match ep_type {
        EndpointType::Version => cb(ep, data),
        EndpointType::Security => sec.security_handler(ep, data),
        EndpointType::Other => {
            let mut data_mut = data.clone();

            sec.decrypt(&mut data_mut);
            let mut res = cb(ep, data_mut);
            sec.encrypt(&mut res);

            res
        }
    }
}
