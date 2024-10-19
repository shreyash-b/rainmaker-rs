use std::cell::Cell;

use uuid::Uuid;

use crate::{
    ble::{ApplicationHandle, Characteristic, Descriptor, GattApplication, Service},
    protocomm::{protocomm_req_handler, ProtocommCallbackType},
    utils::wrap_in_arc_mutex,
};

const USER_CHARACTERISTIC_DESCRIPTOR: u128 = 0x290100001000800000805f9b34fb;

#[derive(Default)]
pub(crate) struct TransportGatt {
    app_handle: Option<ApplicationHandle>,
    characteristics: Cell<Vec<Characteristic>>,
    service_uuid: Uuid,
}

impl TransportGatt {
    pub(crate) fn new(service_uuid: Uuid) -> Self {
        Self {
            service_uuid,
            ..Default::default()
        }
    }

    pub(crate) fn add_endpoint(
        &mut self,
        uuid: Uuid,
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
            uuid,
            read: Some(Box::new(move || {
                let val = value_mutex.lock().unwrap();
                protocomm_req_handler(&ep_name, &val, &cb, &ep_type, &sec)
            })),
            write: Some(Box::new(move |data| {
                let mut val = value_mutex_2.lock().unwrap();
                *val = data;
            })),
            descriptors: vec![Descriptor {
                uuid: Uuid::from_u128(USER_CHARACTERISTIC_DESCRIPTOR),
                value: ep_name_2.into(),
            }],
        };

        let chrs = self.characteristics.get_mut();
        chrs.push(new_characteristic);
    }

    pub(crate) fn start(&mut self) {
        let app = GattApplication {
            services: vec![Service {
                uuid: self.service_uuid,
                primary: true,
                characteristics: self.characteristics.take(), // will work but not desirable
            }],
        };

        self.app_handle = Some(app.serve().unwrap());
    }
}
