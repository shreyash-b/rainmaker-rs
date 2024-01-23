use components::mqtt::{self, MqttEvent};

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
    // let peripherals = esp_idf_svc::hal::peripherals::Peripherals::take().unwrap();
    // let sysloop = esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap();
    // let nvs = esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();

    // let mut wifi = esp_idf_svc::wifi::BlockingWifi::wrap(
    //     esp_idf_svc::wifi::EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs)).unwrap(),
    //     sysloop,
    // ).unwrap();

    // wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(esp_idf_svc::wifi::ClientConfiguration{
    //     ssid: "nothing phone 2".into(),
    //     password: "LOWERCASE".into(),
    //     ..Default::default()
    // })).unwrap();

    // wifi.start().unwrap();
    // wifi.connect().unwrap();

    // esp_idf_svc::hal::delay::Delay::new_default().delay_ms(5000);

    /*                  MQTT Client connection , subscribe, publish                */

    let mut client = mqtt::MqttClient::new(
        &mqtt::MqttConfiguration {
            host: "a1p72mufdu6064-ats.iot.us-east-1.amazonaws.com",
            clientid: "58CF79DA4FD0",
            port: 8883,
        },
        Box::new(|event| match event {
            MqttEvent::Connected => log::info!("MQTT Connected"),
            MqttEvent::Publish(msg) => log::info!(
                "Received value = {}",
                String::from_utf8(msg.payload).unwrap()
            ),
            MqttEvent::Disconnected => log::error!("MQTT Disconnected"),
            MqttEvent::BeforeConnect => log::warn!("MQTT Connecting"),
            MqttEvent::Received => log::info!("Message Published"),
            _ => log::warn!("Unaddressed Event"),
        }),
    )
    .unwrap();

    client.publish(
    "node/58CF79DA4FD0/params/local",
    &mqtt::QoSLevel::AtLeastOnce,
    "{\"Light\":{\"Name\":\"Light\",\"Power\":true,\"Brightness\":40,\"Hue\":270,\"Saturation\":100},}".into()
    );

    client.subscribe(
        "node/58CF79DA4FD0/params/remote",
        &mqtt::QoSLevel::AtLeastOnce,
    );

    loop {
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}
