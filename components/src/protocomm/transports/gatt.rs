#![cfg(target_os = "espidf")]

use std::{marker::PhantomData, sync::Arc};

use esp32_nimble::{
    utilities::mutex::Mutex, BLEAdvertisementData, BLEDevice, BLEService, DescriptorProperties,
    NimbleProperties,
};

use crate::{
    protocomm::{protocomm_req_handler, CallbackData},
    utils::WrappedInArcMutex,
};

use super::TransportCallbackType;

#[derive(Debug, Default)]
pub struct BleConfig {
    pub device_name: String,
}

pub struct TransportGatt<'a> {
    ble_device: &'static BLEDevice,
    cb_data: Option<WrappedInArcMutex<CallbackData>>,
    prim_service: Arc<Mutex<BLEService>>,
    count: u16,
    phantom: PhantomData<&'a str>,
}

impl TransportGatt<'_> {
    pub fn new(config: BleConfig) -> Self {
        let ble_device = BLEDevice::take();
        let server = ble_device.get_server();
        let prim_service = server.create_service(esp32_nimble::utilities::BleUuid::Uuid16(0x6969));

        ble_device
            .get_advertising()
            .lock()
            .set_data(
                BLEAdvertisementData::new()
                    .name(&config.device_name)
                    .add_service_uuid(prim_service.lock().uuid()),
            )
            .unwrap();

        Self {
            ble_device,
            prim_service,
            cb_data: None,
            count: 0,
            phantom: Default::default(),
        }
    }

    pub(crate) fn register_cb_data(&mut self, data: WrappedInArcMutex<CallbackData>) {
        self.cb_data = Some(data)
    }

    pub(crate) fn add_endpoint(&mut self, ep_name: &str, cb: impl TransportCallbackType) {
        let cb_data = self.cb_data.clone().unwrap();
        self.count += 1;

        let name = ep_name.to_string();
        let curr_characteristic = self.prim_service.lock().create_characteristic(
            esp32_nimble::utilities::BleUuid::Uuid16(0x2020 + self.count),
            NimbleProperties::READ | NimbleProperties::WRITE,
        );

        curr_characteristic
            .lock()
            .on_read(move |val, _| {
                let res = protocomm_req_handler(
                    name.clone(),
                    val.value().to_vec(),
                    cb.clone(),
                    cb_data.clone(),
                );
                val.set_value(&res);
            })
            .create_descriptor(
                esp32_nimble::utilities::BleUuid::Uuid16(0x2901),
                DescriptorProperties::READ,
            )
            .lock()
            .set_value(ep_name.as_bytes());
    }

    pub(crate) fn advertise(&self) {
        self.ble_device.get_advertising().lock().start().unwrap();
    }
}
