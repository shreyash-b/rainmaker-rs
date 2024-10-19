use crate::{error::Error, protocomm::ProtocommCallbackType};

pub trait WiFiProvTransportTrait {
    fn start(&mut self) -> Result<(), Error>;

    fn add_endpoint(&mut self, ep_name: &str, cb: ProtocommCallbackType);
    fn set_version_info(&mut self, ep_name: &str, version_info: String);
    fn set_security_ep(&mut self, ep_name: &str);
}
