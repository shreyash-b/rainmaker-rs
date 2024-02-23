use rainmaker::node::*;

pub fn create_light_device(name: &str) -> Device{
    let mut light_device = Device::new(name, DeviceType::Lightbulb, "Power", vec![]);
    let power_param = Param::new_power("Power");
    let saturation_param = Param::new_satuation("Saturation");
    let hue_param = Param::new_hue("Hue");
    let brightness_param = Param::new_brighness("Brightness"); 
    
    light_device.add_param(power_param);
    light_device.add_param(hue_param);
    light_device.add_param(saturation_param);
    light_device.add_param(brightness_param);

    light_device
}

pub fn create_led_device(name: &str) -> Device{
    let mut led_device = Device::new(name, DeviceType::Light, "Power", vec![]);
    let power_param = Param::new_power("Power");
    let brightness_param = Param::new_brighness("Brightness");
    
    led_device.add_param(power_param);
    led_device.add_param(brightness_param);

    led_device
}