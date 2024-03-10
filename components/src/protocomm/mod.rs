mod proto;
mod security;
pub mod transports;

use crate::error::Error;
pub use prost::Message;
pub use proto::*; // temporarily
use std::marker::PhantomData;
use transports::httpd::TransportHttpd;

use transports::TransportTrait;

use self::security::{sec0, SecurityTrait};

pub enum ProtocomTransport<'a> {
    Httpd(TransportHttpd<'a>),
    Pd(PhantomData<&'a ()>),
}

impl ProtocomTransport<'_> {
    fn start(&self) {
        match self {
            ProtocomTransport::Httpd(_) => {},
            ProtocomTransport::Pd(_) => todo!(),
        }
    }
}

pub struct ProtocommConfig<'a> {
    _security: ProtocommSecurity,
    _transport: ProtocomTransport<'a>,
}

pub struct Protocomm<'a> {
    transport: ProtocomTransport<'a>,
    security: ProtocommSecurity,
}

#[derive(Debug)]
pub enum ProtocommSecurity {
    Sec0,
}

// type EndpointCallback = dyn Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static;
impl<'a> Protocomm<'a> {
    pub fn new(transport: ProtocomTransport<'a>, security: ProtocommSecurity) -> Self {
        Self {
            transport,
            security,
        }
    }

    pub fn register_endpoint<T>(&self, ep_name: &str, callback: T) -> Result<(), Error>
    where
        T: Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static,
    {
        match &self.transport {
            ProtocomTransport::Httpd(t) => {
                t.add_endpoint(ep_name, callback);
            }
            _ => return Ok(()),
        }

        Ok(())
    }

    pub fn set_security_endpoint(&self, ep_name: &str) -> Result<(), Error> {
        let sec_endpoint = match self.security {
            ProtocommSecurity::Sec0 => sec0::Sec0::security_handler,
        };

        self.register_endpoint(ep_name, sec_endpoint)?;
        Ok(())
    }

    pub fn start(&self) {
        self.transport.start();
    }
}

pub(crate) fn protocomm_req_handler<T>(ep: String, data: Vec<u8>, cb: T) -> Vec<u8>
where
    T: Fn(String, Vec<u8>) -> Vec<u8>,
{
    // handle encryption, decryption after implementing Sec1/Sec2
    cb(ep, data)
}
