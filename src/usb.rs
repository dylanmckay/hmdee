use {Error, ResultExt};
use {command, protocol, sensor};

use std::ptr;
use std;
use std::time::Duration;

use libusb;

pub const PSVR_VID: u16 = 0x054c;
pub const PSVR_PID: u16 = 0x09af;

pub type ByteOrder = ::byteorder::LittleEndian;

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

/// A PSVR USB device.
pub struct Psvr<'a> {
    context: &'a libusb::Context,
    device: libusb::Device<'a>,
    device_desc: libusb::DeviceDescriptor,
    handle: libusb::DeviceHandle<'a>,
}

#[derive(Debug)]
pub struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8
}

/// Get an iterator over all PSVRs on the system.
pub fn iter(context: &libusb::Context) -> Result<Iter, Error> {
    let devices: Vec<_> = context.devices()?.iter().collect();

    Ok(Iter {
        context: context,
        devices: devices.into_iter(),
    })
}

/// An iterator over PSVR USB devices.
pub struct Iter<'a> {
    context: &'a libusb::Context,
    devices: std::vec::IntoIter<libusb::Device<'a>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<Psvr<'a>, Error>;

    fn next(&mut self) -> Option<Result<Psvr<'a>, Error>> {
        loop {
            match self.devices.next() {
                Some(device) => {
                    let device_desc = match device.device_descriptor() {
                        Ok(device) => device,
                        Err(_) => continue
                    };

                    if device_desc.vendor_id() == PSVR_VID &&
                        device_desc.product_id() == PSVR_PID {
                        break Some(Psvr::open(self.context, device, device_desc));
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a> Psvr<'a> {
    /// Opens a PSVR device.
    pub fn open(context: &'a libusb::Context,
            device: libusb::Device<'a>,
            device_desc: libusb::DeviceDescriptor) -> Result<Self, Error> {
        let handle = device.open()?;

        let mut psvr = Psvr {
            context, device, device_desc, handle,
        };
        psvr.initialize()?;

        Ok(psvr)
    }

    /// Prints information about the usb device to stdout.
    pub fn print_information(&self) -> Result<(), Error> {
        let timeout = Duration::from_secs(1);
        let languages = self.handle.read_languages(timeout)?;

        println!("Active configuration: {}", self.handle.active_configuration()?);
        println!("Languages: {:?}", languages);

        if languages.len() > 0 {
            let language = languages[0];

            println!("Manufacturer: {:?}", self.handle.read_manufacturer_string(language, &self.device_desc, timeout).ok());
            println!("Product: {:?}", self.handle.read_product_string(language, &self.device_desc, timeout).ok());
            println!("Serial Number: {:?}", self.handle.read_serial_number_string(language, &self.device_desc, timeout).ok());
        }

        Ok(())
    }

    /// Sends a command.
    pub fn send_command<C>(&mut self,
                           command: &C) -> Result<(), Error>
        where C: command::Command {
        let payload = command.payload_bytes();

        assert!(payload.len() <= protocol::COMMAND_PAYLOAD_SIZE,
                "command payload too large for protocol");

        // Convert payload from slice to array.
        let mut temp_payload = [0; protocol::COMMAND_PAYLOAD_SIZE];
        unsafe {
            ptr::copy(payload.as_ptr(), temp_payload.as_mut_ptr(), 1);
        }
        let payload = temp_payload;

        // Build command with specified ID and payload.
        let command = protocol::Command {
            header: protocol::CommandHeader {
                id: C::ID,
                magic: 0xAA,
                status: 0,
                length: payload.len() as u8,
            },
            payload: payload,
        };
        let raw_command = unsafe { ::std::slice::from_raw_parts(
                &command as *const _ as *const u8,
                ::std::mem::size_of_val(&command))
        };

        self.send_raw(raw_command).chain_err(|| "could not send command")
    }

    /// Initialises the PSVR.
    fn initialize(&mut self) -> Result<(), Error> {
        self.handle.reset()?;

        let config_desc = self.config_desc();

        for &interface_number in usb_interfaces::INTERFACES_TO_CLAIM {
            let interface = config_desc.interfaces()
                .nth(interface_number as usize)
                .expect("could not find interface");

            match self.handle.kernel_driver_active(interface.number()) {
                Ok(true) => if self.context.supports_detach_kernel_driver() {
                    self.handle.detach_kernel_driver(interface.number()).ok();
                } else {
                    eprintln!("cannot detach the kernel driver on this platform");
                },
                _ => ()
            }
        }

        // FIXME: do this once we're done with the endpoint fully (outside of this function).
        // if has_kernel_driver {
        //     self.handle.attach_kernel_driver(endpoint.iface).ok();
        // }

        Ok(())
    }

    /// Sends raw data.
    fn send_raw(&mut self,
                data: &[u8]) -> Result<(), Error> {
        let timeout = Duration::from_secs(1);

        let config_desc = self.config_desc();

        let interface = config_desc.interfaces()
            .nth(usb_interfaces::HID_CONTROL as usize)
            .expect("could not find interface");

        let interface_desc = interface.descriptors()
            .next()
            .expect("could not find interface descriptor");

        let endpoint_desc = interface_desc.endpoint_descriptors()
            .filter(|e| e.direction() == libusb::Direction::Out)
            .next()
            .expect("could not get endpoint desc");

        self.handle.write_bulk(endpoint_desc.address(), data, timeout)?;
        Ok(())
    }

    pub fn receive_sensor(&mut self) -> Result<sensor::Frame, Error> {
        use self::sensor::Readable;

        let timeout = Duration::from_secs(1);

        let config_desc = self.config_desc();

        let interface = config_desc.interfaces()
            .nth(usb_interfaces::HID_SENSOR as usize)
            .expect("could not find interface");

        let interface_desc = interface.descriptors()
            .next()
            .expect("could not find interface descriptor");

        let endpoint_desc = interface_desc.endpoint_descriptors()
            .filter(|e| e.direction() == libusb::Direction::In)
            .next()
            .expect("could not get endpoint desc");

        let mut buf: [u8; sensor::FRAME_SIZE] = [0; sensor::FRAME_SIZE];
        let bytes_read = self.handle.read_interrupt(endpoint_desc.address(), &mut buf, timeout).chain_err(|| "could not read from device")?;

        if bytes_read != sensor::FRAME_SIZE {
            panic!("not enough bytes read of sensor frame");
        }

        let frame = sensor::Frame::read_bytes(&mut buf)?;

        Ok(frame)
    }

    /// Sets whether the VR is powered or not.
    pub fn set_power(&mut self, on: bool) -> Result<(), Error> {
        self.send_command(&command::SetPower { on })
    }

    fn config_desc(&self) -> libusb::ConfigDescriptor {
        self.device.config_descriptor(0).expect("could not get first config descriptor")
    }
}

