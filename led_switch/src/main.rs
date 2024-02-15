fn main() {
    rainmaker::rainmaker_init();
    rainmaker::rainmaker_say_hello();

    /* ESP specific code : WiFi library integration to be done... */

    // let peripherals = esp_idf_svc::hal::peripherals::Peripherals::take().unwrap();
    // let sysloop = esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap();
    // let nvs = esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();

    // let mut wifi = esp_idf_svc::wifi::BlockingWifi::wrap(
    // esp_idf_svc::wifi::EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs)).unwrap(),
    // sysloop,
    // )
    // .unwrap();

    // wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(
    // esp_idf_svc::wifi::ClientConfiguration {
    // ssid: "nothing phone 2".into(),
    // password: "LOWERCASE".into(),
    // ..Default::default()
    // },
    // ))
    // .unwrap();

    // wifi.start().unwrap();
    // wifi.connect().unwrap();

    // esp_idf_svc::hal::delay::Delay::new_default().delay_ms(5000);

    /* --------------------------------------------------------------------------------------- */

    let mut client = rainmaker::mqtt::mqtt_init();

    let param = rainmaker::node::Params::new(
        "Power",
        "bool",
        vec!["read".to_owned(), "write".to_owned()],
        "esp.ui.toggle",
    );
    let mut device = rainmaker::node::Devices::new(
        "Light",
        rainmaker::node::DeviceType::Lightbulb,
        "Power",
        vec![],
        vec![],
    );
    let mut _node = rainmaker::node::Node::new(
        "58CF79DA4FD0".to_owned(),
        "2019-02-27".to_owned(),
        rainmaker::node::Info {},
        vec![],
        vec![],
    );

    rainmaker::node::Devices::add_param(&mut device, param);

    rainmaker::node::Node::add_device(&mut _node, device);

    let node_config = serde_json::to_string(&_node).unwrap();

    log::info!("Node Config : {}", &node_config);

    rainmaker::mqtt::mqtt_publish(&mut client, "node/58CF79DA4FD0/config", &node_config);

    rainmaker::mqtt::mqtt_subscribe(&mut client, "node/58CF79DA4FD0/params/remote");

    loop {
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}
