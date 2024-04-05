#![cfg(target_os = "espidf")]

use embedded_svc::ota::Ota;
use esp_idf_svc::ota::EspOta;

use crate::{error::Error, http_client};

use super::base::*;


impl OTAHandler<esp_idf_svc::ota::EspOta> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(EspOta::new()?))
    }

    pub fn get_image_data(&mut self, url: &str) -> Result<(), Error> {
        let mut ota_update = self.0.initiate_update()?;
        

        let mut http_client = http_client::HttpClient::new()?;
        let request = http_client.client.get(url)?;
        let mut response = request.submit()?;

        match response.header("Content-Length") {
            Some(img_len) => {
                log::info!("Image Length: {}", img_len);
            }
            None => {
                log::warn!("No image loaded");
            }
        }

        let mut total_bytes_read: u64 = 0u64;

        let mut buffer = [0u8; 1024];

        loop {
            let bytes_read = match response.read(buffer.as_mut()) {
                Ok(bytes_read) => bytes_read,
                Err(err) => {
                    log::error!("Error in reading data: {}", err);
                    return Err(err.into());
                }
            };

            if bytes_read == 0 {
                continue;
            }

            if total_bytes_read % 10240 == 0 {
                log::info!("Image bytes read: {}", total_bytes_read);
            }
            ota_update.write(buffer.as_ref())?;
            total_bytes_read += bytes_read as u64;

            if total_bytes_read == embedded_svc::http::Headers::content_len(&response).unwrap() {
                break;
            }
        }

        log::info!("Total bytes read: {}", total_bytes_read);
        log::warn!("Completing ota update");
        ota_update.complete()?;
        log::warn!("OTA update complete...Rebooting in 10 secs");

        esp_idf_hal::delay::Delay::new_default().delay_ms(10000);

        esp_idf_hal::reset::restart();
        

        Ok(())
    }

    pub fn mark_partition_valid(&mut self) -> Result<(), Error> {
        self.0.mark_running_slot_valid()?;
        self.0.initiate_update()?.complete()?;  
        Ok(())
    }

    pub fn mark_partition_invalid_and_rollback(&mut self) {
        self.0.mark_running_slot_invalid_and_reboot();
    }

    pub fn mark_partition_valid_pending(&mut self) -> bool {
        let boot_slot = self.0.get_boot_slot().unwrap();
        match boot_slot.state{
            esp_idf_svc::ota::SlotState::Factory | esp_idf_svc::ota::SlotState::Valid => {
                false
            },
            _ => true
        }
    }
}
