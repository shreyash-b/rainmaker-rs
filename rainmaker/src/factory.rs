pub mod constants {
    pub const FACTORY_PARTITION_NAME: &str = "fctry";
    pub const FACTORY_NAMESPACE_NAME: &str = "rmaker_creds";

    pub const FACTORY_NODE_ID_KEY: &str = "node_id";
    pub const FACTORY_CERT_KEY: &str = "client_cert";
    pub const FACTORY_PRIV_KEY: &str = "client_key";
}

pub mod rmaker_factory {
    use super::constants::*;

    use components::persistent_storage::{Nvs, NvsPartition};
    use std::sync::LazyLock;

    use crate::error::RMakerError;

    static FACTORY_PARTITION: LazyLock<NvsPartition> = LazyLock::new(|| {
        NvsPartition::new(FACTORY_PARTITION_NAME)
            .expect("Unable to intialize factory partition. Is it created?")
    });

    // is not mutable so can be only used for reads
    static FACTORY_NVS: LazyLock<Nvs> = LazyLock::new(|| {
        Nvs::new(FACTORY_PARTITION.clone(), FACTORY_NAMESPACE_NAME)
            .expect("Failed to create namespace for factory partition")
    });

    pub fn get_bytes(key: &str) -> Option<Vec<u8>> {
        FACTORY_NVS.get_bytes(key)
    }

    pub fn set_bytes(key: &str, bytes: &[u8]) -> Result<(), RMakerError> {
        let mut nvs = Nvs::new(FACTORY_PARTITION.clone(), FACTORY_NAMESPACE_NAME)?;
        let _ = nvs.set_bytes(key, bytes)?;
        Ok(())
    }
}
