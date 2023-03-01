mod frame;

use rusb;

fn convert_argument(input: &str) -> (u16, u16) {
    let ids = input.split(':').collect::<Vec<_>>();
    if ids.len() != 2 {
        panic!("Invalid input. Make sure it is in the correct format (hex:hex)");
    }
    let mut ret: Vec<u16> = Vec::new();
    for id in ids {
        ret.push(u16::from_str_radix(id, 16)
            .expect("Provide hexadecimal values. Do not add '0x'"));
    }
    return (ret[0], ret[1]);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: rust-keyboard-hack vid:pid");
        return;
    }

    let (target_vid, target_pid) = convert_argument(args[1].as_ref());

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
