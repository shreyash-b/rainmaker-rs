use super::{EndpointType, ProtocommCallbackType, ProtocommSecurity};
use std::sync::Arc;

pub mod httpd;

pub(crate) trait TransportCallbackType = Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static;
pub(crate) trait TransportTrait {
    fn add_endpoint(
        &mut self,
        ep_name: &str,
        cb: ProtocommCallbackType,
        ep_type: EndpointType,
        sec: Arc<ProtocommSecurity>,
    );
}
