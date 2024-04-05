mod proto;
mod security;
pub mod transports;

use crate::{error::Error, http_server::HttpConfiguration};
pub use prost::Message;
pub use proto::*;
use std::collections::HashMap;
use transports::httpd::TransportHttpd;

use transports::TransportTrait;

pub use self::security::ProtocommSecurity;
use self::security::SecurityTrait;
use crate::utils::{wrap_in_arc_mutex, WrappedInArcMutex};

const LOGGER_TAG: &str = "protocomm";

pub enum ProtocomTransportConfig {
    Httpd(HttpConfiguration),
}

pub struct ProtocommConfig {
    pub security: ProtocommSecurity,
    pub transport: ProtocomTransportConfig,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum EndpointType {
    Version,
    Security,
    #[default]
    Other,
}

#[derive(Default)]
pub struct CallbackData {
    sec: ProtocommSecurity,
    ep_ype: HashMap<String, EndpointType>,
}

pub struct Protocomm<'a> {
    transport: TransportHttpd<'a>, // change this to accepting trait after implementing BLE transport
    cb_data: WrappedInArcMutex<CallbackData>,
}

impl<'a> Protocomm<'a> {
    pub fn new(config: ProtocommConfig) -> Self {
        let cb_data = wrap_in_arc_mutex(CallbackData {
            sec: config.security,
            ..Default::default()
        });

        let mut httpd = match config.transport {
            ProtocomTransportConfig::Httpd(http_config) => TransportHttpd::new(http_config),
        };
        httpd.register_cb_data(cb_data.clone());
        Self {
            transport: httpd,
            cb_data,
        }
    }

    pub fn register_endpoint<T>(&mut self, ep_name: &str, callback: T) -> Result<(), Error>
    where
        T: Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static,
    {
        self.register_endpoint_internal(ep_name, callback, EndpointType::Other)
    }

    pub fn set_security_endpoint(&mut self, ep_name: &str) -> Result<(), Error> {
        // supply dummy callback here, appropriate callback is called
        self.register_endpoint_internal(ep_name, |_ep, _data| vec![], EndpointType::Security)
    }

    pub fn set_version_endpoint(
        &mut self,
        ep_name: &str,
        version_str: String,
    ) -> Result<(), Error> {
        self.register_endpoint_internal(
            ep_name,
            move |_ep, _data| version_str.to_owned().into(),
            EndpointType::Version,
        )
    }

    fn register_endpoint_internal<T>(
        &mut self,
        ep_name: &str,
        callback: T,
        ep_type: EndpointType,
    ) -> Result<(), Error>
    where
        T: Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static,
    {
        let mut ep_data = self.cb_data.as_ref().lock().unwrap();
        ep_data.ep_ype.insert(ep_name.to_string(), ep_type);

        self.transport.add_endpoint(ep_name, callback);
        log::debug!(target: LOGGER_TAG, "registered handler for {ep_name}");

        Ok(())
    }
}

pub(crate) fn protocomm_req_handler<T>(
    ep: String,
    data: Vec<u8>,
    cb: T,
    cb_data: WrappedInArcMutex<CallbackData>,
) -> Vec<u8>
where
    T: Fn(String, Vec<u8>) -> Vec<u8>,
{
    let mut cb_data = cb_data.lock().unwrap();
    let curr_ep_type = cb_data.ep_ype.get(&ep).unwrap();

    match curr_ep_type {
        EndpointType::Version => cb(ep, data),
        EndpointType::Security => {
            let sec = &mut cb_data.sec;
            sec.security_handler(ep, data)
        }
        EndpointType::Other => {
            let sec = &mut cb_data.sec;
            let mut data_mut = data.clone();

            sec.decrypt(&mut data_mut);
            let mut res = cb(ep, data_mut);
            sec.encrypt(&mut res);

            res
        }
    }
}
