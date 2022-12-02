use rusb::{
    DeviceList,
    GlobalContext,
};

fn show_device(device_context: DeviceList<GlobalContext>) {    
    
    for device in device_context.iter() {
        let device_desc = device.device_descriptor();
        match device_desc {
            Ok(_v) => {
                println!("Bus {:03}", device.bus_number());
            },
            Err(_e) => continue,
        }

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
