use components::http::*;
use components::local_ctrl::*;
use ::mdns::mdns::*;


fn get_property_values() {
    // Implementation for getting property values
}

fn set_property_values() {
    // Implementation for setting property values
}

pub fn local_ctrl_service() -> anyhow::Result<()> {
    let mut _mdns = MdnsService::mdns_init().unwrap();
    _mdns.mdns_hostname_set("hMKvVdMC8eKt6UzoNHTVvj");

    _mdns.mdns_service_add("hMKvVdMC8eKt6UzoNHTVvj", "esp_local_ctrl", "tcp", &[("node_id", "hMKvVdMC8eKt6UzoNHTVvj"), ("version_endpoint", "/esp_local_ctrl/version"), ("session_endpoint", "/esp_local_ctrl/session"), ("control_endpoint", "/esp_local_ctrl/control")]);


    let http_server_config = HttpConfiguration { port: 8080, ..Default::default()};

    let handlers = LocalCtrlHandlers {
        get_prop_values: get_property_values,
        set_prop_values: set_property_values,
    };

    let mut config = LocalCtrlConfig {
        transport: http_server_config,
        handlers,
        max_properties: 10,
    };

    LocalCtrlConfig::local_ctrl_start(&mut config)
}

