use serde::{Deserialize, Serialize};
use serde_json::{json, Value};



fn main() {
    /*
        To be fixed :-
            1. trait implementation in mqtt abstraction
            2. too many unwarp in the code
            3. better way of error handling
            4. find a way to avoid variables like wifi and client from dropping (avoid use of delay and infinite loop in end main.rs file)
    */

    rainmaker::rainmaker_init();
    rainmaker::rainmaker_say_hello();
    
    /* ESP specific code : WiFi library integration to be done... */

    let peripherals = esp_idf_svc::hal::peripherals::Peripherals::take().unwrap();
    let sysloop = esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap();
    let nvs = esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();

    let mut wifi = esp_idf_svc::wifi::BlockingWifi::wrap(
        esp_idf_svc::wifi::EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs)).unwrap(),
        sysloop,
    ).unwrap();

    wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(esp_idf_svc::wifi::ClientConfiguration{
        ssid: "nothing phone 2".into(),
        password: "LOWERCASE".into(),
        ..Default::default()
    })).unwrap();

    wifi.start().unwrap();
    wifi.connect().unwrap();

    esp_idf_svc::hal::delay::Delay::new_default().delay_ms(5000);


    let mut light = json!({
        "Light":{
            "Name": "Light",
            "Power": false,
            "Brightness": 90
        }
    });  


    /*                  MQTT Client connection , subscribe, publish                */

    let mut client = rainmaker::mqtt::mqtt_init();

    let node_config = serde_json::json!({
        "node_id": "58CF79DA4FD0",
        "config_version": "2019-02-27",
        "info": {},
        "attributes": [],
        "devices": [{
            "name": "Light",
            "type": "esp.device.lightbulb",
            "primary": "Power",
            "params": [{
                "name": "Name",
                "type": "esp.param.name",
                "data_type": "string",
                "properties": ["read", "write"]
            }, {
                "name": "Power",
                "data_type": "bool",
                "properties": ["read", "write", "time_series"],
                "ui_type": "esp.ui.toggle"
            }, {
                "name": "Brightness",
                "data_type": "int",
                "properties": ["read", "write"],
                "bounds": {
                    "min": 0,
                    "max": 100
                },
                "ui_type": "esp.ui.slider"
            }]
        }]
    });

    rainmaker::mqtt::mqtt_publish(&mut client, "node/58CF79DA4FD0/config", &node_config.to_string());

    rainmaker::mqtt::mqtt_subscribe(&mut client, "node/58CF79DA4FD0/params/remote");

    rainmaker::mqtt::mqtt_publish(&mut client, "node/58CF79DA4FD0/params/local", "{\"Light\":{\"Name\":\"Light\",\"Power\":true,\"Brightness\":40},}".into());

    // rainmaker::mqtt::mqtt_publish(&mut client, "node/58CF79DA4FD0/params/local", &light.to_string());

    loop {
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}