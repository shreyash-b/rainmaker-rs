mod base;

mod persistent_esp;
mod persistent_linux;

#[cfg(target_os = "espidf")]
pub type Nvs = base::Nvs<esp_idf_svc::nvs::EspNvs<esp_idf_svc::nvs::NvsCustom>>;

#[cfg(target_os = "linux")]
pub type Nvs = base::Nvs<pickledb::PickleDb>;

#[cfg(target_os = "espidf")]
pub type NvsPartition = base::NvsPartition<esp_idf_svc::nvs::EspCustomNvsPartition>;

#[cfg(target_os = "linux")]
pub type NvsPartition = base::NvsPartition<std::path::PathBuf>;
