use rusb::{
    DeviceList,
    GlobalContext,
    DeviceDescriptor,
    Device,
    DeviceHandle,
};

fn print_usb_desc(device: Device<GlobalContext>, device_handle: DeviceHandle<GlobalContext>, device_desc: DeviceDescriptor) {
   let product_id = device_desc.product_id();
   let vendor_id = device_desc.vendor_id();
   let bus_number = device.bus_number();
   let address = device.address();
   
    let manufacturer_string = 
        match device_handle.read_manufacturer_string_ascii(&device_desc) {
            Ok(_v) => _v,
            Err(_e) => "".to_owned(),
        };
    
    let product_string =
        match device_handle.read_product_string_ascii(&device_desc) {
            Ok(_v) => _v,
            Err(_e) => "".to_owned(),
        };
   
    println!("USB Version: {:03}", device_desc.usb_version());
    println!("Bus: {:03}, Device: {:03}, Vendor Id: {:04}, Product Id: {:04}",
    bus_number, address, vendor_id, product_id);
    println!("Manufacturer String: {}", manufacturer_string);
    println!("Product String: {}\n", product_string);
}

fn show_device(device_context: DeviceList<GlobalContext>) {    
    
    for device in device_context.iter() {
        let device_desc = device.device_descriptor();
        match device_desc {
            Ok(_v) => {
                match device.open() {
                    Ok(_handle) => print_usb_desc(device, _handle, _v),
                    Err(_e) => continue,
                }
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
