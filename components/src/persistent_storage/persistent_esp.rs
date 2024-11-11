#![cfg(target_os = "espidf")]

use super::base::*;
use crate::error::Error;
use esp_idf_svc::nvs::{EspCustomNvsPartition, EspNvs, NvsCustom};

impl NvsPartition<EspCustomNvsPartition> {
    pub fn new(partition_name: &str) -> Result<Self, Error> {
        let partition = EspCustomNvsPartition::take(partition_name)?;
        Ok(Self(partition))
    }
}

impl Nvs<EspNvs<NvsCustom>> {
    pub fn new(
        partition: NvsPartition<EspCustomNvsPartition>,
        namespace: &str,
    ) -> Result<Self, Error> {
        Ok(Self(EspNvs::new(partition.0, namespace, true)?))
    }

    pub fn remove(&mut self, key: &str) -> Result<bool, Error> {
        Ok(self.0.remove(key)?)
    }

    pub fn set_u8(&mut self, name: &str, data: u8) -> Result<(), Error> {
        self.0.set_u8(name, data)?;
        Ok(())
    }

    pub fn set_bytes(&mut self, name: &str, bytes: &[u8]) -> Result<(), Error> {
        self.0.set_blob(name, bytes)?;
        Ok(())
    }

    pub fn set_str(&mut self, name: &str, val: &str) -> Result<(), Error> {
        self.0.set_str(name, val)?;
        Ok(())
    }

    pub fn get_u8(&self, key: &str) -> Result<Option<u8>, Error> {
        self.0.get_u8(key).map_err(|x| x.into())
    }

    pub fn get_bytes(&self, name: &str, buff: &mut [u8]) -> Result<Option<Vec<u8>>, Error> {
        let ret = self.0.get_blob(name, buff)?;
        Ok(ret.map(|x| x.to_vec()))
    }

    pub fn get_string(&self, name: &str, buff: &mut [u8]) -> Result<Option<String>, Error> {
        let ret = self.0.get_str(name, buff)?;
        Ok(ret.map(|x| x.to_string()))
    }
}
