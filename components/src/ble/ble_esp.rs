#![cfg(target_os = "espidf")]

use std::sync::LazyLock;

use esp32_nimble::{
    utilities::BleUuid, BLEAdvertisementData, BLEDevice, DescriptorProperties, NimbleProperties,
};

use crate::{ble::base::*, error::Error};

pub type ApplicationHandle = ApplicationHandleGeneric<()>;

static BLE_DEVICE: LazyLock<&'static mut BLEDevice> = LazyLock::new(|| BLEDevice::take());

impl Advertisement {
    pub fn advertise(self) -> Result<AdvertisementHandle, Error> {
        let mut adv_data = BLEAdvertisementData::from(self);
        let mut adv = BLE_DEVICE.get_advertising().lock();
        if adv.set_data(&mut adv_data).is_err() {
            return Err(Error("Unable to set BLE advertising data".into()));
        }
        if adv.start().is_err() {
            return Err(Error("Failed to start BLE advertising".into()));
        }
        Ok(AdvertisementHandle {})
    }
}

impl From<Advertisement> for BLEAdvertisementData {
    fn from(value: Advertisement) -> Self {
        let mut adv = Self::new();
        adv.name(&value.device_name.unwrap_or_default());
        value.service_uuids.iter().for_each(|x| {
            adv.add_service_uuid(BleUuid::from(*x));
        });
        adv
    }
}

impl Drop for AdvertisementHandle {
    fn drop(&mut self) {
        let mut adv = BLE_DEVICE.get_advertising().lock();
        adv.reset().unwrap(); // no chance for failure here
    }
}

impl Application {
    pub fn serve(self) -> Result<ApplicationHandle, Error> {
        let server = BLE_DEVICE.get_server();
        for srv in self.services {
            let current_service_mutex = server.create_service(srv.uuid.into());
            let mut current_service = current_service_mutex.lock();

            for chr in srv.characteristics {
                let mut chr_properties = NimbleProperties::empty();
                if chr.read.is_some() {
                    chr_properties |= NimbleProperties::READ;
                }
                if chr.write.is_some() {
                    chr_properties |= NimbleProperties::WRITE;
                }
                let curr_characteristic_mutex =
                    current_service.create_characteristic(chr.uuid.into(), chr_properties);
                let mut curr_characteristic = curr_characteristic_mutex.lock();

                if chr.read.is_some() {
                    curr_characteristic
                        .on_read(move |val, _desc| val.set_value(&chr.read.as_ref().unwrap()()));
                }

                if chr.write.is_some() {
                    curr_characteristic
                        .on_write(move |arg| chr.write.as_ref().unwrap()(arg.recv_data().to_vec()));
                }

                chr.descriptors.iter().for_each(|x| {
                    let descriptor_mutex = curr_characteristic
                        .create_descriptor(BleUuid::from(x.uuid), DescriptorProperties::READ);
                    let mut descriptor = descriptor_mutex.lock();
                    descriptor.set_value(&x.value);
                });
            }
        }
        Ok(ApplicationHandleGeneric(()))
    }
}

impl<T> Drop for ApplicationHandleGeneric<T> {
    fn drop(&mut self) {
        todo!();
        // Umm, do this before merge
    }
}
