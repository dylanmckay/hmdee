extern crate libusb;
extern crate byteorder;

pub mod protocol;

use std::collections::HashMap;
use std::ptr;
use std::time::Duration;
use byteorder::{ByteOrder, LittleEndian};

pub const PSVR_VID: u16 = 0x054c;
pub const PSVR_PID: u16 = 0x09af;

#[allow(dead_code)]
mod usb_interfaces {
    pub type InterfaceAddress = u8;

	pub const AUDIO_3D: InterfaceAddress = 0;
	pub const AUDIO_CONTROL: InterfaceAddress = 1;
	pub const AUDIO_MIC: InterfaceAddress = 2;
	pub const AUDIO_CHAT: InterfaceAddress = 3;
	pub const HID_SENSOR: InterfaceAddress = 4;
	pub const HID_CONTROL: InterfaceAddress = 5;
	pub const VS_H264: InterfaceAddress = 6;
	pub const VS_BULK_IN: InterfaceAddress = 7;
	pub const HID_CONTROL2: InterfaceAddress = 8;

    pub const INTERFACES_TO_CLAIM: &'static [InterfaceAddress] = &[
        HID_SENSOR,
        HID_CONTROL,
    ];
}

#[derive(Debug)]
pub struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8
}

pub fn test() {
    match libusb::Context::new() {
        Ok(mut context) => {
            context.set_log_level(libusb::LogLevel::Debug);
            let (mut device, device_desc, mut handle) = match open_device(&context, PSVR_VID, PSVR_PID) {
                Some(a) => a,
                None => panic!("could not find device {:04x}:{:04x}", PSVR_VID, PSVR_PID)
            };

            run_device(&context, &mut device, &device_desc, &mut handle).unwrap()
        },
        Err(e) => panic!("could not initialize libusb: {}", e)
    }
}

fn open_device(context: &libusb::Context, vid: u16, pid: u16) -> Option<(libusb::Device, libusb::DeviceDescriptor, libusb::DeviceHandle)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(_) => continue
            }
        }
    }

    None
}

fn run_device(context: &libusb::Context,
              device: &mut libusb::Device,
              device_desc: &libusb::DeviceDescriptor,
              handle: &mut libusb::DeviceHandle) -> libusb::Result<()> {
    try!(handle.reset());

    let timeout = Duration::from_secs(1);
    let languages = try!(handle.read_languages(timeout));

    println!("Active configuration: {}", try!(handle.active_configuration()));
    println!("Languages: {:?}", languages);

    if languages.len() > 0 {
        let language = languages[0];

        println!("Manufacturer: {:?}", handle.read_manufacturer_string(language, device_desc, timeout).ok());
        println!("Product: {:?}", handle.read_product_string(language, device_desc, timeout).ok());
        println!("Serial Number: {:?}", handle.read_serial_number_string(language, device_desc, timeout).ok());
    }

    let config_desc = device.config_descriptor(0).expect("could not get first config descriptor");
    let mut claimed_interfaces: HashMap<usb_interfaces::InterfaceAddress, libusb::InterfaceDescriptor> = HashMap::new();

    for &interface_number in usb_interfaces::INTERFACES_TO_CLAIM {
        let interface = config_desc.interfaces()
            .nth(interface_number as usize)
            .expect("could not find interface to claim");

        for interface_desc in interface.descriptors() {
            assert_eq!(interface_number, interface_desc.interface_number());

            match handle.kernel_driver_active(interface_desc.interface_number()) {
                Ok(true) => if context.supports_detach_kernel_driver() {
                    handle.detach_kernel_driver(interface_desc.interface_number()).ok();
                } else {
                    eprintln!("cannot detach the kernel driver on this platform");
                },
                _ => ()
            }

            handle.claim_interface(interface_desc.interface_number()).unwrap();
            claimed_interfaces.insert(interface_desc.interface_number(), interface_desc);

        }
    }

    set_power(handle, &claimed_interfaces, true);

    // FIXME: do this once we're done with the endpoint fully (outside of this function).
    // if has_kernel_driver {
    //     handle.attach_kernel_driver(endpoint.iface).ok();
    // }

    Ok(())
}

// int psvr_send_command_sync(psvr_context *ctx, uint8_t id, uint8_t *payload, uint32_t length)
// {
// 	struct morpheus_control_command command;
// 	int ep;
// 	int xferred;
// 	int err;
//
// 	command.header.id = id;
// 	command.header.magic = 0xAA;
// 	command.header.length = length;
// 	memcpy(command.payload, payload, length);
//
// 	ep = ctx->usb_descriptor->interface[PSVR_INTERFACE_HID_CONTROL]
// 		.altsetting[0]
// 		.endpoint[0]
// 		.bEndpointAddress;
// 	ep &= ~ENDPOINT_IN;
//
// 	err = libusb_bulk_transfer(ctx->usb_handle, ep, (uint8_t *) &command,
// 		length + MORPHEUS_COMMAND_HEADER_SIZE, &xferred, 0);
//
//     return err;
// }

pub fn send_command(handle: &mut libusb::DeviceHandle,
                    interface_descs: &HashMap<usb_interfaces::InterfaceAddress, libusb::InterfaceDescriptor>,
                    id: u8,
                    payload: &[u8]) {
    assert!(payload.len() <= protocol::COMMAND_PAYLOAD_SIZE,
            "command payload too large for protocol");

    // Convert payload from slice to array.
    let mut temp_payload = [0; protocol::COMMAND_PAYLOAD_SIZE];
    unsafe {
        ptr::copy(payload.as_ptr(), temp_payload.as_mut_ptr(), 1);
    }
    let payload = temp_payload;

    let command = protocol::Command {
        header: protocol::CommandHeader {
            id,
            magic: 0xAA,
            status: 0,
            length: payload.len() as u8,
        },
        payload,

    };
    let raw_command = unsafe {
        ::std::slice::from_raw_parts(
            &command as *const _ as *const u8,
            ::std::mem::size_of_val(&command))
    };

    let timeout = Duration::from_secs(1);

    let interface_desc = interface_descs.get(&usb_interfaces::HID_CONTROL)
        .expect("could not get hid control interface desc");

    let endpoint_desc = interface_desc.endpoint_descriptors()
        .filter(|e| e.direction() == libusb::Direction::Out)
        .next()
        .expect("could not get endpoint desc");

    handle.write_bulk(endpoint_desc.address(), raw_command, timeout).unwrap();
}

pub fn set_power(handle: &mut libusb::DeviceHandle,
                 interface_descs: &HashMap<usb_interfaces::InterfaceAddress, libusb::InterfaceDescriptor>,
                 on: bool) {
    println!("Set power: {}", on);
    let mut bytes: [u8; 4] = [0; 4];
    LittleEndian::write_u32(&mut bytes, if on { 1 } else { 0 });
    // return psvr_send_command_sync(ctx, 0x17, (uint8_t *) &on, 4);
    send_command(handle, interface_descs, 0x17, &bytes);
}
