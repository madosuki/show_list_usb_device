fn main() {
    let devices = rusb::devices();
    
    for device in devices.unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03}", device.bus_number());
    }
}
