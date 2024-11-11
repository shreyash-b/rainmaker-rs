#![cfg(target_os = "linux")]

use std::{fs, path::PathBuf};

use pickledb::PickleDb;

use super::base::*;
use crate::error::Error;

impl NvsPartition<PathBuf> {
    pub fn new(partition_name: &str) -> Result<Self, Error> {
        let username = std::env::var("USER").unwrap();
        let config_dir = format!("/home/{}/.config/rmaker/", username);

        let config_dir_pathbuf = PathBuf::from(config_dir);
        if !config_dir_pathbuf.exists() {
            fs::create_dir(&config_dir_pathbuf)?;
        }

        let partition_path = config_dir_pathbuf.join(partition_name);
        if !partition_path.exists() {
            Err(Error("partition not found".to_string()))?
        }

        Ok(Self(partition_path))
    }
}

impl Nvs<PickleDb> {
    pub fn new(partition: NvsPartition<PathBuf>, namespace: &str) -> Result<Self, Error> {
        let namespace_pathname = format!("{}.json", namespace);
        let namespace_path = partition.0.join(namespace_pathname);

        let db = if namespace_path.exists() {
            PickleDb::load(
                &namespace_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Json,
            )?
        } else {
            PickleDb::new(
                &namespace_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Json,
            )
        };

        Ok(Self(db))
    }

    pub fn remove(&mut self, key: &str) -> Result<bool, Error> {
        Ok(self.0.rem(key)?)
    }

    pub fn set_u8(&mut self, key: &str, data: u8) -> Result<(), Error> {
        Ok(self.0.set(key, &data)?)
    }

    pub fn set_bytes(&mut self, key: &str, bytes: &[u8]) -> Result<(), Error> {
        Ok(self.0.set(key, &bytes)?)
    }

    pub fn set_str(&mut self, key: &str, val: &str) -> Result<(), Error> {
        Ok(self.0.set(key, &val)?)
    }

    pub fn get_u8(&self, key: &str) -> Option<u8> {
        self.0.get::<u8>(key)
    }

    pub fn get_bytes(&self, key: &str, buff: &mut [u8]) -> Result<Option<Vec<u8>>, Error> {
        let data: Option<Vec<u8>> = self.0.get(key);
        if let Some(mut data_int) = data {
            data_int.shrink_to(buff.len());
            return Ok(Some(data_int));
        }
        Ok(None)
    }

    pub fn get_string(&self, key: &str, buff: &mut [u8]) -> Result<Option<String>, Error> {
        let data: Option<String> = self.0.get(key);
        if let Some(mut data_int) = data {
            data_int.shrink_to(buff.len());
            return Ok(Some(data_int));
        }
        Ok(None)
    }
}
