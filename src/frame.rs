// このコードの大部分はrusb公式githubのexamplesを参考にしている

use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, TransferType, UsbContext,
};
use std::time::Duration;

#[derive(Debug, Clone)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
    protocol_code: u8,
}

enum InterfaceProcol {
    Device = 0,
    Keyboard = 1,
    Mouse = 2,
}

pub fn read_ascii_array<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: DeviceDescriptor,
    handle: &mut DeviceHandle<GlobalContext>,
    transfer_type: TransferType,
    buf: &mut [u8],
) -> Result<usize, String> {
    let endpoints = match find_readable_endpoint(device, device_desc, transfer_type)
    {
        Some(endpoints) => endpoints,
        None => return Err(String::from("endpoint not found")),
    };

    let mut result = Ok(0);

    let mut result_mouse = Ok(0);

    for endpoint in endpoints.to_vec() {
        // endpoint_address is expected to be 82
        println!("endpoint address: 0x{:4x}", endpoint.address);
        println!("endpoint: {:?}", endpoint);


        // これを実行しないとOS側のデータ転送要求と競合して？I/Oエラーが出る　https://github.com/libusb/libusb/wiki/FAQ#user-content-Does_libusb_support_USB_HID_devices
        let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
            Ok(true) => {
                println!("set_auto_detach_kernel_driver: {:?}", handle.set_auto_detach_kernel_driver(false));
                println!("detach_kernel_driver: {:?}", handle.detach_kernel_driver(endpoint.iface));
                true
            }
            _ => false,
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("has kernel driver? {}", has_kernel_driver);
        
    }
    for endpoint in endpoints.to_vec() {
        if endpoint.protocol_code != InterfaceProcol::Device as u8 {
            continue;
        }
        result = match configure_endpoint(handle, &endpoint) {
            Ok(_) => {
                    let timeout = Duration::from_secs(1);
                    // match handle.read_interrupt(endpoint.address, buf, timeout) {
                    //     Ok(n_byte) => Ok(n_byte),
                    //     Err(e) => Err(format!("read_interrupt failed: {:?}", e)),
                    // }
                    Ok(0)
            },
            Err(err) => Err(format!("could not configure endpoint{:x}: {}", endpoint.address, err)),
        };

        if endpoint.protocol_code == 2 {
            result_mouse = result.clone();
        }

    }

    for endpoint in endpoints {
        // if has_kernel_driver == true {
            println!("attach_kernel_driver: {:?}", handle.attach_kernel_driver(endpoint.iface));
            // println!("release_interface: {:?}", handle.release_interface(endpoint.iface));
        // }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }


    std::thread::sleep(std::time::Duration::from_secs(3));
    result_mouse
}

// この関数は正しく実装されていることが保証されている
fn find_readable_endpoint<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: DeviceDescriptor,
    transfer_type: TransferType,
) -> Option<Vec<Endpoint>> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut endpoints: Vec<Endpoint> = Vec::new();

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == Direction::In
                        && endpoint_desc.transfer_type() == transfer_type
                    {
                        endpoints.push(Endpoint {
                            config: config_desc.number(),
                            iface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                            address: endpoint_desc.address(),
                            protocol_code: interface_desc.protocol_code(),
                        });
                    }
                }
            }
        }

        return Some(endpoints);
    }
    None
}

// attach_kernel_driverをするときに以下を実行しないと正しくドライバが適用されない
// OSに接続されたUSBデバイスを一旦リセットし，libusbに接続されたデバイスとして初期化，再宣言，再設定する．
fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> rusb::Result<()> {
    // ?演算子：　Errorを受け取ると"""即座に"""returnする => claim_interfaceとset_alternate_settingが呼ばれていなかった
    handle.set_active_configuration(endpoint.config)?; // "issue a SET_CONFIGURATION request using the current configuration, causing most USB-related device state to be reset (altsetting reset to zero, endpoint halts cleared, toggles reset)."
    // // source: https://github.com/libusb/libusb/blob/9e077421b8708d98c8d423423bd6678dca0ef2ae/libusb/core.c#L1733
    // // Resource Busy
    // handle.claim_interface(endpoint.iface)?; // "You must claim the interface you wish to use before you can perform I/O on any of its endpoints. instruct the underlying operating system that your application wishes to take ownership of the interface."
    // source: https://github.com/libusb/libusb/blob/9e077421b8708d98c8d423423bd6678dca0ef2ae/libusb/core.c#L1770
    // handle.set_alternate_setting(endpoint.iface, endpoint.setting)?; // Activate an alternate setting for an interface.
    // source: https://github.com/libusb/libusb/blob/9e077421b8708d98c8d423423bd6678dca0ef2ae/libusb/core.c#L1859
    // Entity not found
    Ok(()) // could not configure endpoint: Resource busy
    // dmesg: "usbfs: interface 0 claimed by usbhid while 'rust-keyboard-h' sets config #1"
    // You cannot change/reset configuration if other applications or drivers have claimed interfaces.
}
