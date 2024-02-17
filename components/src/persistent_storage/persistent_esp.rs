#![cfg(target_os="espidf")]

use super::base::*;
use crate::error::Error;
use esp_idf_svc::nvs::{EspCustomNvsPartition, EspNvs, NvsCustom};

impl NvsPartition<EspCustomNvsPartition>{
    pub fn new(partition_name: &str) -> Result<Self, Error>{
        let partition = EspCustomNvsPartition::take(&partition_name.to_string())?;
        Ok(Self(partition))
    }
}

impl Nvs<EspNvs<NvsCustom>>{
    pub fn new(partition:NvsPartition<EspCustomNvsPartition>, namespace: &str) -> Result<Self, Error>{
        Ok(Self(EspNvs::new(partition.0, namespace, true)?))
    }

    pub fn set_bytes(&mut self, name: &str, bytes: &[u8]) -> Result<(), Error>{
        self.0.set_blob(name, bytes)?;
        Ok(())
    }

    pub fn get_bytes<'a>(&self, name: &str) -> Option<Vec<u8>>{
        let mut buf = [0; 2500];
        let data = self.0.get_blob(name, &mut buf).unwrap().to_owned();
        match data {
            Some(v) => Some(v.to_vec()),
            None => None,
        }
    }
}