use std::{cell::Cell, marker::PhantomData};

use uuid::Uuid;

use crate::{
    ble::{
        Advertisement, AdvertisementHandle, Application, ApplicationHandle, Characteristic,
        Descriptor, Service,
    },
    protocomm::{protocomm_req_handler, ProtocommCallbackType},
    utils::wrap_in_arc_mutex,
};

use super::TransportTrait;

#[derive(Default)]
pub struct TransportBleConfig {
    pub device_name: String,
    pub service_uuid: Uuid,
}

#[derive(Default)]
pub(crate) struct TransportBle<'a> {
    adv_handle: Option<AdvertisementHandle>,
    app_handle: Option<ApplicationHandle>,
    characteristics: Cell<Vec<Characteristic>>,
    count: u16,
    device_name: String,
    service_uuid: Uuid,
    phantom_data: PhantomData<&'a ()>,
}

impl TransportBle<'_> {
    pub fn new(config: TransportBleConfig) -> Self {
        Self {
            device_name: config.device_name,
            service_uuid: config.service_uuid,
            ..Default::default()
        }
    }
}

impl TransportTrait for TransportBle<'_> {
    fn add_endpoint(
        &mut self,
        ep_name: &str,
        cb: ProtocommCallbackType,
        ep_type: crate::protocomm::EndpointType,
        sec: std::sync::Arc<crate::protocomm::ProtocommSecurity>,
    ) {
        let ep_name = ep_name.to_string();
        let ep_name_2 = ep_name.clone();
        let value_mutex = wrap_in_arc_mutex(vec![]);
        let value_mutex_2 = value_mutex.clone();
        let new_characteristic = Characteristic {
            uuid: Uuid::from_u128((0x1212 + self.count) as u128),
            read: Some(Box::new(move || {
                let val = value_mutex.lock().unwrap();
                protocomm_req_handler(&ep_name, val.to_vec(), &cb, &ep_type, &sec)
            })),
            write: Some(Box::new(move |data| {
                let mut val = value_mutex_2.lock().unwrap();
                *val = data;
            })),
            descriptors: vec![Descriptor {
                uuid: Uuid::from_u128(0x290100001000800000805f9b34fb), // User Description Characteristic
                value: ep_name_2.into(),
            }],
        };

        let chrs = self.characteristics.get_mut();
        chrs.push(new_characteristic);
        self.count += 1;
    }

    fn start(&mut self) {
        let adv = Advertisement {
            device_name: Some(self.device_name.clone()),
            discoverable: true,
            primary_service: self.service_uuid,
            service_uuids: vec![self.service_uuid],
        };

        let app = Application {
            services: vec![Service {
                uuid: self.service_uuid,
                primary: true,
                characteristics: self.characteristics.take(), // will work but not desirable
            }],
        };

        self.app_handle = Some(app.serve().unwrap());
        self.adv_handle = Some(adv.advertise().unwrap())
    }
}
