#![cfg(target_os = "linux")]
use std::sync::{LazyLock, Mutex};

use crate::ble::base::*;
use crate::error::Error;
use bluer::adv;
use bluer::gatt::local;

pub type ApplicationHandle = ApplicationHandleGeneric<local::ApplicationHandle>;

static RUNTIME: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

static BLUER_SESSION: LazyLock<bluer::Session> =
    LazyLock::new(|| RUNTIME.block_on(bluer::Session::new()).unwrap());

static ADAPTER: LazyLock<bluer::Adapter> = LazyLock::new(|| {
    RUNTIME
        .block_on(BLUER_SESSION.default_adapter())
        .expect("Unable to initialize BLE: Adapter not found")
});

static ADVERTISEMENT: Mutex<Option<adv::AdvertisementHandle>> = Mutex::new(None);

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

impl Application {
    pub fn serve(self) -> Result<ApplicationHandle, Error> {
        let handle = RUNTIME.block_on(ADAPTER.serve_gatt_application(self.into()))?;
        Ok(ApplicationHandleGeneric(handle))
    }
}

impl From<Application> for local::Application {
    fn from(value: Application) -> Self {
        let services = value.services.into_iter().map(|x| x.into()).collect();
        Self {
            services,
            ..Default::default()
        }
    }
}

impl Advertisement {
    pub fn advertise(self) -> Result<AdvertisementHandle, Error> {
        let mut global_adv = match ADVERTISEMENT.lock() {
            Ok(val) => val,
            Err(_) => return Err(Error("Unable to acquire global advertisement lock".into())),
        };

        if global_adv.is_some() {
            return Err(Error("Advertisement already set".into()));
        }
        let new_handle = RUNTIME.block_on(ADAPTER.advertise(self.into()))?;
        *global_adv = Some(new_handle);
        Ok(AdvertisementHandle {})
    }
}

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

impl Drop for AdvertisementHandle {
    fn drop(&mut self) {
        // Take out the handle which will then be dropped, thereby stopping the advertisement
        ADVERTISEMENT.lock().unwrap().take();
    }
}
