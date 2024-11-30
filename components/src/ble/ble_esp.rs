#![cfg(target_os = "espidf")]

use esp32_nimble::{
    utilities::BleUuid, BLEAdvertisementData, BLEDevice, DescriptorProperties, NimbleProperties,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    LazyLock,
};

use crate::{ble::base::*, error::Error};

pub type ApplicationHandle = ApplicationHandleGeneric<()>;
pub type AdvertisementHandle = AdvertisementHandleGeneric<()>;

static BLE_DEVICE: LazyLock<&'static mut BLEDevice> = LazyLock::new(|| BLEDevice::take());

static ADVERTISING: AtomicBool = AtomicBool::new(false);
static SERVING: AtomicBool = AtomicBool::new(false);

impl Advertisement {
    pub fn advertise(self) -> Result<AdvertisementHandleGeneric<()>, Error> {
        if ADVERTISING.load(Ordering::SeqCst) {
            return Err(Error("Already advertising".to_string()));
        }

        let mut adv_data = BLEAdvertisementData::from(self);
        let mut adv = BLE_DEVICE.get_advertising().lock();
        if adv.set_data(&mut adv_data).is_err() {
            return Err(Error("Unable to set BLE advertising data".into()));
        }
        if adv.start().is_err() {
            return Err(Error("Failed to start BLE advertising".into()));
        }

        ADVERTISING.store(true, Ordering::SeqCst);

        Ok(AdvertisementHandleGeneric(()))
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

impl<T> Drop for AdvertisementHandleGeneric<T> {
    fn drop(&mut self) {
        BLEDevice::deinit_full().expect("Unable to deinit BLE");
        ADVERTISING.store(false, Ordering::SeqCst);
    }
}

impl GattApplication {
    pub fn serve(self) -> Result<ApplicationHandle, Error> {
        if SERVING.load(Ordering::SeqCst) {
            return Err(Error("Already serving.".to_string()));
        }

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

                if let Some(read_fn) = chr.read {
                    curr_characteristic.on_read(move |val, _desc| val.set_value(&read_fn()));
                }

                if let Some(write_fn) = chr.write {
                    curr_characteristic.on_write(move |arg| write_fn(arg.recv_data().to_vec()));
                }

                chr.descriptors.iter().for_each(|x| {
                    let descriptor_mutex = curr_characteristic
                        .create_descriptor(BleUuid::from(x.uuid), DescriptorProperties::READ);
                    let mut descriptor = descriptor_mutex.lock();
                    descriptor.set_value(&x.value);
                });
            }
        }

        SERVING.store(true, Ordering::SeqCst);
        Ok(ApplicationHandleGeneric(()))
    }
}
