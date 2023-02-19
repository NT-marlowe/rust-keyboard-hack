mod frame;
use rusb;
use std::time::Duration;

fn main() {
    // let target_vid: u16 = 0x0c45;
    // let target_pid: u16 = 0x7680;
    for device in rusb::devices().unwrap().iter() {
        let device_descriptor = device.device_descriptor().unwrap();

        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_descriptor.vendor_id(),
            device_descriptor.product_id()
        );

        let handle = match device.open() {
            Ok(handle) => handle,
            Err(e) => panic!("{}", e),
        };

        let timeout = Duration::from_secs(1);
        let language = handle.read_languages(timeout).unwrap()[0];

        let product_str = match handle.read_product_string(language, &device_descriptor, timeout) {
            Ok(s) => s,
            Err(_) => String::from("unable to read string descriptor"),
        };

        println!("Product: {}", product_str);
        println!("-----------------------------")
    }
}
