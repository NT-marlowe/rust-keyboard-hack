use rusb::{Device, DeviceDescriptor, Direction, TransferType, UsbContext};

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
