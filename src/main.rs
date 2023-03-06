mod frame;

use rusb;

fn convert_argument(input: &str) -> (u8, u8, u8) {
    let ids = input.split('.').collect::<Vec<_>>();
    if ids.len() != 3 {
        panic!("Invalid input. Make sure it is in the correct format: hex.hex.hex");
    }
    let mut ret: Vec<u8> = Vec::new();
    for id in ids {
        ret.push(u8::from_str_radix(id, 16)
            .expect("Provide hexadecimal values. Do not add '0x'"));
    }
    return (ret[0], ret[1], ret[2]);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: rust-keyboard-hack Bus.Device.EndPoint");
        return;
    }

    let (target_bus_num, target_address, target_endpoint) = convert_argument(args[1].as_ref());

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

        if device.bus_number() != target_bus_num
            || device.address() != target_address
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
            target_endpoint,
        ) {
            Ok(n) => {
                println!("n is {}", n);
                println!("Received data: {:?}", &buf[0..32]);
                println!("Received data: {:?}", &buf[32..64]);
                println!("Received data: {:?}", &buf[64..96]);
                println!("Received data: {:?}", &buf[96..128]);
                println!("Received data: {:?}", &buf[128..160]);
                println!("Received data: {:?}", &buf[160..192]);
                println!("Received data: {:?}", &buf[192..224]);
                println!("Received data: {:?}", &buf[224..256]);
            },
            Err(e) => panic!("{}", e),
        }
    }
}
