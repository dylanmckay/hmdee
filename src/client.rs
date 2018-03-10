use {usb, Error, ErrorKind, ResultExt};
use {command, inertia, math, protocol, sensor};

use std;
use hidapi;
use na;

/// A PSVR device connected via USB.
///
/// * `'a'` is the lifetime
pub struct Psvr<'a> {
    /// The USB HID control interface.
    control_device: hidapi::HidDevice<'a>,
    /// The USB HID sensor interface.
    sensor_device: hidapi::HidDevice<'a>,
    /// The inertia sensor.
    inertia_sensor: inertia::Sensor,
}

/// Get an iterator over all PSVRs on the system.
pub fn iter(hidapi: &hidapi::HidApi) -> Result<Iter, Error> {
    Ok(Iter {
        psvr_infos: discover::all(hidapi)?.into_iter(),
        hidapi,
    })
}

/// An iterator over PSVR USB devices.
pub struct Iter<'a> {
    hidapi: &'a hidapi::HidApi,
    psvr_infos: std::vec::IntoIter<discover::PsvrInfo>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<Psvr<'a>, Error>;

    fn next(&mut self) -> Option<Result<Psvr<'a>, Error>> {
        loop {
            match self.psvr_infos.next() {
                Some(psvr_info) => {
                    break Some(Psvr::connect(&psvr_info, self.hidapi));
                },
                None => break None,
            }
        }
    }
}

/// Opens an arbitrary connected PSVR device.
pub fn get(hidapi: &hidapi::HidApi)
    -> Result<Option<Psvr>, Error> {
    match iter(hidapi)?.next() {
        Some(psvr) => Ok(Some(psvr?)),
        None => Ok(None),
    }
}

impl<'a> Psvr<'a> {
    /// Connects to a discovered PSVR device.
    fn connect(psvr_info: &discover::PsvrInfo,
               hidapi: &'a hidapi::HidApi) -> Result<Self, Error> {
        let control_device_info = psvr_info.interface_device_info(usb::Interface::HidControl)
            .expect("PSVR does not expose an HID control interface");
        let sensor_device_info = psvr_info.interface_device_info(usb::Interface::HidSensor)
            .expect("PSVR does not expose an HID sensor interface");

        let control_device = hidapi.open_path(&control_device_info.path).unwrap(); // FIXME: remove unwrap.
        let sensor_device = hidapi.open_path(&sensor_device_info.path).unwrap(); // FIXME: remove unwrap.
        Ok(Psvr {
            inertia_sensor: inertia::Sensor::new(),
            control_device, sensor_device,
        })
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

        self.send_raw(&command.raw_bytes()).chain_err(|| "could not send command")
    }

    /// Sends raw data.
    fn send_raw(&mut self,
                data: &[u8]) -> Result<(), Error> {
        self.control_device.write(&data.to_owned())?;
        Ok(())
    }

    /// Receives sensor data.
    pub fn receive_sensor(&mut self) -> Result<sensor::Readout, Error> {
        use self::sensor::Readable;

        loop {
            let mut buf: [u8; sensor::FRAME_SIZE] = [0; 64];
            let bytes_read = match self.sensor_device.read_timeout(&mut buf, 1) {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    let err: Error = ErrorKind::Hid(e).into();
                    return Err(err).chain_err(|| "could not read from device");
                },
            };


            if bytes_read <= 1 {
                continue; // We need more than the report ID.
            } if bytes_read != sensor::FRAME_SIZE {
                panic!("not enough bytes read of sensor readout (expected {} bytes but got {} bytes)", sensor::FRAME_SIZE, bytes_read);
            }

            let readout = sensor::Readout::read_bytes(&buf)?;

            // FIXME: perhaps we should interpolate between the thingse
            for instant in readout.instants.iter().take(1) {
                let (g,a) = (instant.gyroscope(), instant.accelerometer());

                self.inertia_sensor.update(&inertia::Instant {
                    gyroscope: na::Vector3::new(g.x as _, g.y as _, g.z as _),
                    accelerometer: na::Vector3::new(a.x as _, a.y as _, a.z as _),
                });
            }
            return Ok(readout);
        }
    }

    /// Powers on the PSVR.
    pub fn power_on(&mut self) -> Result<(), Error> {
        self.set_power(true)
    }

    /// Powers off the PSVR.
    pub fn power_off(&mut self) -> Result<(), Error> {
        self.set_power(false)
    }

    /// Sets the state of the power.
    pub fn set_power(&mut self, on: bool) -> Result<(), Error> {
        self.send_command(&command::SetPower { on }).chain_err(|| "could not send set power command")
    }

    pub fn vr_mode(&mut self) -> Result<(), Error> {
        self.send_command(&command::SetVrMode { vr_mode: true }).chain_err(|| "could not enable vr mode")
    }

    /// Enables VR trawcking.
    pub fn vr_tracking(&mut self) -> Result<(), Error> {
        self.send_command(&command::EnableVrTracking)
    }

    /// Powers off the PSVR and disconnects from it.
    pub fn close(mut self) -> Result<(), Error> {
        self.send_command(&command::SetPower { on: false }).map(|_| ())
    }

    /// Gets the orientation of the PSVR headset.
    pub fn orientation(&self) -> math::Quaternion {
        self.inertia_sensor.hmd_orientation()
    }
}

mod discover {
    use Error;
    use usb;

    use hidapi;

    /// Information about an individual PSVR USB interface.
    ///
    /// Note that HIDAPI models interfaces are individual devices.
    #[derive(Debug)]
    pub struct InterfaceInfo {
        pub interface: usb::Interface,
        pub device_info: hidapi::HidDeviceInfo,
    }

    #[derive(Debug)]
    pub struct PsvrInfo {
        pub interfaces: Vec<InterfaceInfo>,
    }

    /// Gets information about every connected PSVR device.
    ///
    // FIXME: currently this assumes that only one PSVR is plugged in.
    //        Need to group interfaces by USB device.
    pub fn all(hidapi: &hidapi::HidApi) -> Result<::std::vec::IntoIter<PsvrInfo>, Error> {
        let interface_devices: Vec<_> = hidapi.devices().into_iter().filter(|device_info| {
            device_info.vendor_id == usb::PSVR_VID && device_info.product_id == usb::PSVR_PID
        }).collect();

        if !interface_devices.is_empty() {
            let interfaces: Result<Vec<_>, Error> = interface_devices.into_iter().map(|hid_device| {
                Ok(InterfaceInfo {
                    interface: usb::Interface::from_i32(hid_device.interface_number)?,
                    device_info: hid_device,
                })
            }).collect();
            let interfaces = interfaces?;

            Ok(vec![PsvrInfo { interfaces }].into_iter())
        } else {
            Ok(vec![].into_iter())
        }
    }

    impl PsvrInfo {
        /// Gets the associated HIDAPI device for a usb interface.
        pub fn interface_device_info(&self, interface: usb::Interface) -> Option<&hidapi::HidDeviceInfo> {
            self.interfaces.iter().filter_map(|i| if i.interface == interface { Some(&i.device_info) } else { None }).next()
        }
    }
}

