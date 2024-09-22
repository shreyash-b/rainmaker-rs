pub mod gatt;
pub mod httpd;

#[cfg(target_os = "espidf")]
use gatt::TransportGatt;
use httpd::TransportHttpd;

use super::CallbackData;
use crate::utils::WrappedInArcMutex;

pub(crate) trait TransportCallbackType =
    Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + Clone + 'static; // TODO: remove this clone later

pub(crate) enum ProtocommTransport<'a> {
    #[cfg(target_os = "espidf")]
    Ble(TransportGatt<'a>),
    Httpd(TransportHttpd<'a>),
}

impl ProtocommTransport<'_> {
    pub fn register_cb_data(&mut self, data: WrappedInArcMutex<CallbackData>) {
        match self {
            #[cfg(target_os = "espidf")]
            ProtocommTransport::Ble(transport_gatt) => transport_gatt.register_cb_data(data),
            ProtocommTransport::Httpd(transport_httpd) => transport_httpd.register_cb_data(data),
        }
    }

    pub fn add_endpoint(&mut self, ep_name: &str, cb: impl TransportCallbackType) {
        match self {
            #[cfg(target_os = "espidf")]
            ProtocommTransport::Ble(transport_gatt) => transport_gatt.add_endpoint(ep_name, cb),
            ProtocommTransport::Httpd(transport_httpd) => transport_httpd.add_endpoint(ep_name, cb),
        }
    }

    pub fn start(&self) {
        #[cfg(target_os = "espidf")]
        if let Self::Ble(transport_gatt) = self {
            transport_gatt.advertise();
        };
    }
}
