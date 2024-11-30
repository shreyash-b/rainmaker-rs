use uuid::Uuid;

use crate::{
    ble::{Advertisement, AdvertisementHandle},
    error::Error,
    protocomm::{ProtocommCallbackType, ProtocommGatt, ProtocommGattConfig, ProtocommSecurity},
};

use super::base::WiFiProvTransportTrait;

#[derive(Debug, Default, Clone)]
pub struct WifiProvBleConfig {
    pub service_name: String,
    pub service_uuid: [u8; 16],
}

pub struct WiFiProvTransportBle {
    pub(crate) device_name: String,
    pub(crate) uuid: [u8; 16],
    pub(crate) ep_count: u16,
    pub(crate) pc: ProtocommGatt,
    pub(crate) adv_handle: Option<AdvertisementHandle>,
}

impl WiFiProvTransportBle {
    pub fn new(config: WifiProvBleConfig, sec: ProtocommSecurity) -> Self {
        let pc = ProtocommGatt::new(
            ProtocommGattConfig {
                service_uuid: Uuid::from_bytes(config.service_uuid),
            },
            sec,
        );

        Self {
            device_name: config.service_name,
            uuid: config.service_uuid,
            ep_count: 0,
            pc,
            adv_handle: None,
        }
    }
}

impl WiFiProvTransportTrait for WiFiProvTransportBle {
    fn start(&mut self) -> Result<(), Error> {
        self.pc.start();
        self.start_advertising()?;

        Ok(())
    }

    fn add_endpoint(&mut self, ep_name: &str, cb: ProtocommCallbackType) {
        let uuid = self.next_uuid();

        self.pc.register_endpoint(uuid, ep_name, cb);
    }

    fn set_version_info(&mut self, ep_name: &str, version_info: String) {
        let uuid = self.next_uuid();
        self.pc.set_version_info(uuid, ep_name, version_info);
    }

    fn set_security_ep(&mut self, ep_name: &str) {
        let uuid = self.next_uuid();
        self.pc.set_security_endpoint(uuid, ep_name);
    }
}

impl WiFiProvTransportBle {
    fn start_advertising(&mut self) -> Result<(), Error> {
        let adv = Advertisement {
            device_name: Some(self.device_name.clone()),
            discoverable: true,
            primary_service: Uuid::from_bytes(self.uuid),
            service_uuids: vec![Uuid::from_bytes(self.uuid)],
        };

        let adv_handle = adv.advertise()?;

        self.adv_handle = Some(adv_handle);

        Ok(())
    }

    fn next_uuid(&mut self) -> Uuid {
        let mut new_uuid = self.uuid;
        new_uuid[2] = (self.ep_count >> 8) as u8; // Higher 8 bits
        new_uuid[3] = (self.ep_count & 0xFF) as u8; // Lower 8 bits
        self.ep_count += 1;

        Uuid::from_bytes(new_uuid)
    }
}
