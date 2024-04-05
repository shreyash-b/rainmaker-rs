#![cfg(target_os = "linux")]

use crate::error::Error;

use super::base::*;

impl OTAHandler<()> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(()))
    }
    pub fn get_image_data(&mut self, _url: &str) -> Result<(), Error> {
        Ok(())
    }
    pub fn mark_partition_valid(&mut self) -> Result<(), Error> {
        Ok(())
    }
    pub fn mark_partition_invalid_and_rollback(&mut self) {
    }
    pub fn mark_partition_valid_pending(&mut self) -> bool {
        return false
    }
}