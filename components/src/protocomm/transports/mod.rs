use super::{EndpointType, ProtocommCallbackType, ProtocommSecurity};
use std::sync::Arc;

pub mod ble;
pub mod httpd;

pub(crate) trait TransportTrait {
    fn start(&mut self) {}

    fn add_endpoint(
        &mut self,
        ep_name: &str,
        cb: ProtocommCallbackType,
        ep_type: EndpointType,
        sec: Arc<ProtocommSecurity>,
    );
}
