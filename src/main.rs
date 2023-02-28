mod frame;
use std::thread::sleep_ms;

use rusb;

fn main() {
    let target_vid: u16 = 0x046d;
    let target_pid: u16 = 0xc53f;
    let mut buf: [u8; 256] = [0; 256];
    for mut device in rusb::devices().unwrap().iter() {
        let device_descriptor = device.device_descriptor().unwrap();

        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_descriptor.vendor_id(),
            device_descriptor.product_id()
        );

        if device_descriptor.vendor_id() != target_vid
            || device_descriptor.product_id() != target_pid
        {
            continue;
        }

        let mut handle = match device.open() {
            Ok(handle) => handle,
            Err(e) => panic!("{}", e),
        };

        match frame::read_ascii_array(
            &mut device,
            device_descriptor,
            &mut handle,
            rusb::TransferType::Interrupt,
            &mut buf,
        ) {
            Ok(n) => {
                println!("n is {}", n);
                println!("Received data: {:?}", &buf[0..n]);
            },
            Err(e) => panic!("{}", e),
        }
    }
}
