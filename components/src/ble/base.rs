use uuid::Uuid;

pub trait GattReadFn = Fn() -> Vec<u8> + Send + Sync + 'static;
pub trait GattWriteFn = Fn(Vec<u8>) + Send + Sync + 'static;

pub struct Descriptor {
    pub uuid: Uuid,
    pub value: Vec<u8>,
}

pub struct Characteristic {
    pub uuid: Uuid,
    pub read: Option<Box<dyn GattReadFn>>,
    pub write: Option<Box<dyn GattWriteFn>>,
    pub descriptors: Vec<Descriptor>,
}

pub struct Service {
    pub uuid: Uuid,
    pub primary: bool,
    pub characteristics: Vec<Characteristic>,
}

pub struct Application {
    pub services: Vec<Service>,
}

#[derive(Debug, Default)]
pub struct Advertisement {
    pub device_name: Option<String>,
    pub discoverable: bool,
    pub primary_service: Uuid,
    pub service_uuids: Vec<Uuid>,
}

pub struct AdvertisementHandle;
pub struct ApplicationHandleGeneric<T>(pub(crate) T);
