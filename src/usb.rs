use {Error, ErrorKind, ResultExt};
use {command, protocol, sensor};

use std;

use hidapi;

/// The vendor ID of the PSVR.
const PSVR_VID: u16 = 0x054c;
/// The product ID of the PSVR.
const PSVR_PID: u16 = 0x09af;

/// The byte ordering used by the PSVR.
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
    device: hidapi::HidDevice<'a>,
}

/// Get an iterator over all PSVRs on the system.
pub fn iter(hidapi: &hidapi::HidApi) -> Result<Iter, Error> {
    Ok(Iter {
        hidapi,
        device_infos: hidapi.devices().into_iter(),
        _phantom: std::marker::PhantomData,
    })
}

/// An iterator over PSVR USB devices.
pub struct Iter<'a> {
    hidapi: &'a hidapi::HidApi,
    device_infos: std::vec::IntoIter<hidapi::HidDeviceInfo>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<Psvr<'a>, Error>;

    fn next(&mut self) -> Option<Result<Psvr<'a>, Error>> {
        loop {
            match self.device_infos.next() {
                Some(device_info) => {

                    if device_info.vendor_id == PSVR_VID &&
                        device_info.product_id == PSVR_PID {
                        println!("dev info: {:#?}", device_info);
                        let device = self.hidapi.open_path(&device_info.path).unwrap(); // FIXME: remove unwrap.
                        break Some(Psvr::new(device));
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a> Psvr<'a> {
    pub fn open(hidapi: &'a hidapi::HidApi) -> Result<Option<Self>, Error> {
        let device = hidapi.open(PSVR_VID, PSVR_PID).unwrap();
        Psvr::new(device).map(Some)
    }

    /// Creates a PSVR device.
    fn new(device: hidapi::HidDevice<'a>) -> Result<Self, Error> {
        Ok(Psvr { device })
    }

    /// Prints information about the usb device to stdout.
    pub fn print_information(&self) -> Result<(), Error> {
        // unimplemented!();

        Ok(())
    }

    /// Sends a command.
    pub fn send_command<C>(&mut self,
                           command: &C) -> Result<(), Error>
        where C: command::Command {
        let payload = command.payload_bytes();

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

        let raw_command = command.raw_bytes();

        println!("sending raw {:?}", raw_command);
        self.send_raw(&raw_command).chain_err(|| "could not send command")
    }

    /// Sends raw data.
    fn send_raw(&mut self,
                data: &[u8]) -> Result<(), Error> {
        let mut raw = data.to_owned();
        // Add zero for the report ID.
        raw.insert(0, 0);

        self.device.write(&raw)?;
        Ok(())
    }

    pub fn receive_sensor(&mut self) -> Result<sensor::Frame, Error> {
        use self::sensor::Readable;

        loop {
            let mut buf: [u8; sensor::FRAME_SIZE] = [0; 64];
            let bytes_read = match self.device.read_timeout(&mut buf, 1) {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    let err: Error = ErrorKind::Hid(e).into();
                    return Err(err).chain_err(|| "could not read from device");
                },
            };

            if bytes_read > 0 {
                panic!("bytes read: {:?}", bytes_read);
            }

            // Remove report ID byte from raw data.
            for i in 1..bytes_read {
                buf[i-1] = buf[i];
            }

            if bytes_read <= 1 {
                continue; // We need more than the report ID.
            } if bytes_read != sensor::FRAME_SIZE {
                panic!("not enough bytes read of sensor frame (expected {} bytes but got {} bytes)", sensor::FRAME_SIZE, bytes_read);
            }

            let frame = sensor::Frame::read_bytes(&buf)?;
            return Ok(frame);
        }
    }

    /// Sets whether the VR is powered or not.
    pub fn set_power(&mut self, on: bool) -> Result<(), Error> {
        self.send_command(&command::SetPower { on })
    }

    pub fn vr_tracking(&mut self) -> Result<(), Error> {
        self.send_command(&command::EnableVrTracking)
    }
}

