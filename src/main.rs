use rusb::{
    UsbContext,
    DeviceList,
    GlobalContext,
};

fn show_device(device_context: DeviceList<GlobalContext>) {    
    
    for device in device_context.iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03}", device.bus_number());
    }
}

fn main() {
    let devices = rusb::devices();
    
    match devices {
        Ok(_v) => {
            println!("found!");
            show_device(_v);
        },
        Err(_e) => panic!("{:?}", _e),
    }
}
