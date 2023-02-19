use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, TransferType, UsbContext,
};
use std::time::Duration;

pub fn read_ascii_array<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: DeviceDescriptor,
    handle: DeviceHandle<GlobalContext>,
    transfer_type: TransferType,
    buf: &mut [u8],
) -> Result<usize, String> {
    let endpoint_address = match find_readable_endpoint_address(device, device_desc, transfer_type)
    {
        Some(address) => address,
        None => return Err(String::from("endpoint not found")),
    };

    // endpoint_address is expected to be 81
    let timeout = Duration::from_secs(10);
    match handle.read_interrupt(endpoint_address, buf, timeout) {
        Ok(n_byte) => Ok(n_byte),
        Err(e) => Err(format!("read_interrupt failed: {:?}", e)),
    }
}

// この関数は正しく実装されていることが保証されている
fn find_readable_endpoint_address<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: DeviceDescriptor,
    transfer_type: TransferType,
) -> Option<u8> {
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
                    {
                        return Some(endpoint_desc.address());
                    }
                }
            }
        }
    }
    None
}
