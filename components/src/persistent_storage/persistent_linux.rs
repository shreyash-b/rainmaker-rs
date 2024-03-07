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
        let namespace_path = PathBuf::from(partition.0.join(namespace_pathname));

        let db: PickleDb;
        if namespace_path.exists() {
            db = PickleDb::load(
                &namespace_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Json,
            )?;
        } else {
            db = PickleDb::new(
                &namespace_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Json,
            );
        }

        Ok(Self(db))
    }

    pub fn set_bytes(&mut self, key: &str, bytes: &[u8]) -> Result<(), Error> {
        Ok(self.0.set(key, &bytes)?)
    }

    pub fn get_bytes(&self, key: &str) -> Option<Vec<u8>> {
        self.0.get(key)
    }
}
