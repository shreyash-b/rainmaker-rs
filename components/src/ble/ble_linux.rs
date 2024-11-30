#![cfg(target_os = "linux")]
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::LazyLock;

use crate::ble::base::*;
use crate::error::Error;
use bluer::adv;
use bluer::gatt::local;

pub type ApplicationHandle = ApplicationHandleGeneric<local::ApplicationHandle>;
pub type AdvertisementHandle = AdvertisementHandleGeneric<adv::AdvertisementHandle>;

static RUNTIME: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

static BLUER_SESSION: LazyLock<bluer::Session> =
    LazyLock::new(|| RUNTIME.block_on(bluer::Session::new()).unwrap());

static ADAPTER: LazyLock<bluer::Adapter> = LazyLock::new(|| {
    RUNTIME
        .block_on(BLUER_SESSION.default_adapter())
        .expect("Unable to initialize BLE: Is Bluetooth powered on?")
});

/* Advertising and serving can only be done at one place at a time */
static ADVERTISING: AtomicBool = AtomicBool::new(false);
static SERVING: AtomicBool = AtomicBool::new(false);

impl From<Advertisement> for adv::Advertisement {
    fn from(value: Advertisement) -> Self {
        let service_uuids = value.service_uuids.into_iter().collect();
        Self {
            advertisement_type: adv::Type::Peripheral,
            service_uuids,
            discoverable: Some(value.discoverable),
            local_name: value.device_name,
            ..Default::default()
        }
    }
}

impl From<Descriptor> for local::Descriptor {
    fn from(value: Descriptor) -> Self {
        let read = Some(local::DescriptorRead {
            read: true,
            fun: Box::new(move |_req| {
                let val = value.value.clone();
                Box::pin(async { Ok(val) })
            }),
            ..Default::default()
        });

        Self {
            uuid: value.uuid,
            read,
            ..Default::default()
        }
    }
}

impl From<Characteristic> for local::Characteristic {
    fn from(value: Characteristic) -> Self {
        let uuid = value.uuid;

        let read = value.read.map(|read_func| local::CharacteristicRead {
            read: true,
            fun: Box::new(move |_| {
                let val = read_func();
                Box::pin(async { Ok(val) })
            }),
            ..Default::default()
        });

        let write = value.write.map(|write_func| local::CharacteristicWrite {
            write: true,
            method: local::CharacteristicWriteMethod::Fun(Box::new(move |data, _req| {
                write_func(data);
                Box::pin(async { Ok(()) })
            })),
            ..Default::default()
        });

        let descriptors = value.descriptors.into_iter().map(|x| x.into()).collect();

        Self {
            uuid,
            read,
            write,
            descriptors,
            ..Default::default()
        }
    }
}

impl From<Service> for local::Service {
    fn from(value: Service) -> Self {
        let uuid = value.uuid;
        let primary = value.primary;
        let characteristics = value
            .characteristics
            .into_iter()
            .map(move |x| x.into())
            .collect();
        Self {
            uuid,
            primary,
            characteristics,
            ..Default::default()
        }
    }
}

impl GattApplication {
    pub fn serve(self) -> Result<ApplicationHandle, Error> {
        if SERVING.load(Ordering::SeqCst) {
            return Err(Error("Already serving an application".to_string()));
        }

        let handle = RUNTIME.block_on(ADAPTER.serve_gatt_application(self.into()))?;
        SERVING.store(true, Ordering::SeqCst);
        Ok(ApplicationHandleGeneric(handle))
    }
}

impl From<GattApplication> for local::Application {
    fn from(value: GattApplication) -> Self {
        let services = value.services.into_iter().map(|x| x.into()).collect();
        Self {
            services,
            ..Default::default()
        }
    }
}

impl Advertisement {
    pub fn advertise(self) -> Result<AdvertisementHandle, Error> {
        if ADVERTISING.load(Ordering::SeqCst) {
            return Err(Error("Already Advertising".into()));
        };

        let new_handle = RUNTIME.block_on(ADAPTER.advertise(self.into()))?;
        ADVERTISING.store(true, Ordering::SeqCst);
        Ok(AdvertisementHandleGeneric(new_handle))
    }
}

impl<T> Drop for ApplicationHandleGeneric<T> {
    fn drop(&mut self) {
        SERVING.store(false, Ordering::SeqCst);
    }
}

impl<T> Drop for AdvertisementHandleGeneric<T> {
    fn drop(&mut self) {
        ADVERTISING.store(false, Ordering::SeqCst);
    }
}
