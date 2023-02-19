use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, TransferType, UsbContext,
};
use std::time::Duration;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

pub fn read_ascii_array<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: DeviceDescriptor,
    handle: &mut DeviceHandle<GlobalContext>,
    transfer_type: TransferType,
    buf: &mut [u8],
) -> Result<usize, String> {
    let endpoint = match find_readable_endpoint(device, device_desc, transfer_type)
    {
        Some(endpoint) => endpoint,
        None => return Err(String::from("endpoint not found")),
    };

    // endpoint_address is expected to be 82
    println!("endpoint address: 0x{:4x}", endpoint.address);

    let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(endpoint.iface).ok();
            true
        }
        _ => false,
    };
    println!("has kernel driver? {}", has_kernel_driver);

    let timeout = Duration::from_secs(5);
    let result = match handle.read_interrupt(endpoint.address, buf, timeout) {
        Ok(n_byte) => Ok(n_byte),
        Err(e) => Err(format!("read_interrupt failed: {:?}", e)),
    };

    if has_kernel_driver {
        handle.attach_kernel_driver(endpoint.iface).ok();
    }

    result
}

// この関数は正しく実装されていることが保証されている
fn find_readable_endpoint<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: DeviceDescriptor,
    transfer_type: TransferType,
) -> Option<Endpoint> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == Direction::In
                        && endpoint_desc.transfer_type() == transfer_type
                            && interface_desc.protocol_code() == 2 // Mouse
                    {
                        return Some(Endpoint {
                            config: config_desc.number(),
                            iface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                            address: endpoint_desc.address(),
                        });
                    }
                }
            }
        }
    }
    None
}
