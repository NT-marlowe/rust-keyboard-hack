mod frame;

use std::fs::File;
use std::time::Duration;
use std::path::Path;
use std::io::{self, BufRead};

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

fn decode_protocol(buf: & [u8], product_name: String) -> String {
    let mut buf_binary = String::new();
    for byte in buf {
        buf_binary.push_str(
            &format!("{byte:08b}")
        );
    }
    let str_hid_data = 
        format!("./keyboard/{pn}/{pn}-hid-data.txt", pn=product_name)
    ;

    let mut pushed_key = String::from("");

    if let Ok(lines) = read_lines(str_hid_data) {
        for line in lines {
            if let Ok(data) = line {
                let mut split_data = data.split(',').into_iter();
                let (hid_data, key) = (split_data.next().unwrap(), split_data.next().unwrap());
                if hid_data == buf_binary {
                    pushed_key = key.to_string();
                    break;
                }
            }
        }
    }
    pushed_key
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: rust-keyboard-hack Bus.Device.EndPoint");
        return;
    }

    let (target_bus_num, target_address, target_endpoint) = convert_argument(args[1].as_ref());

    for mut device in rusb::devices().unwrap().iter() {
        let device_descriptor = device.device_descriptor().unwrap();

        print!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_descriptor.vendor_id(),
            device_descriptor.product_id()
        );

        let mut handle = match device.open() {
            Ok(handle) => handle,
            Err(e) => panic!("{}", e),
        };

        let timeout = Duration::from_millis(100);
        let product_name = 
            handle.read_product_string(
                handle.read_languages(timeout).unwrap()[0],
                &device_descriptor,
                timeout
            ).unwrap();

        println!(" Name {}", product_name);

        if device.bus_number() != target_bus_num
            || device.address() != target_address
        {
            continue;
        }

        let bufsize = match product_name.as_str() {
            "DZ60" => 32,
            _ => 8,
        };
        let mut buf: [u8; 256] = [0; 256];

        match frame::read_ascii_array(
            &mut device,
            device_descriptor,
            &mut handle,
            rusb::TransferType::Interrupt,
            &mut buf[0..bufsize],
            target_endpoint,
        ) {
            Ok(_) => {
                // println!("{:?}", buf);
                println!("{}", decode_protocol(&buf[0..bufsize], product_name));
            },
            Err(e) => panic!("{}", e),
        }
    }
}
