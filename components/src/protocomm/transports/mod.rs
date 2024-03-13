pub mod httpd;

pub(crate) trait TransportCallbackType = Fn(String, Vec<u8>) -> Vec<u8> + Send + Sync + 'static;
pub(crate) trait TransportTrait {

    fn add_endpoint(&self, ep_name: &str, cb: impl TransportCallbackType);
}