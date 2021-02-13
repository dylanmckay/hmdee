//! The supported set of PSVR commands;

use std::io::prelude::*;
use std::io;

use crate::usb::ByteOrder;
use byteorder::{WriteBytesExt};

/// A command that can be sent to the PSVR.
pub trait Command {
    const ID: u8;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()>;

    /// Gets the raw bytes that make up the payload.
    fn payload_bytes(&self) -> Vec<u8> {
        let mut buffer = io::Cursor::new(Vec::new());

        self.write_payload(&mut buffer).expect("encountered IO error while writing to memory");
        buffer.into_inner()
    }
}

/// Tells the PSVR to turn power off or on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetPower {
    /// `1` for on, `0` for off.
    pub on: bool,
}

/// Enables VR tracking.
// NOTE: This command can probably be generalised to a 'set tracking on/off' command somehow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnableVrTracking;

/// Enables or disables VR mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetVrMode {
    /// Whether VR mode is disabled.
    pub vr_mode: bool,
}

/// Turns the black box off.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoxOff;

/// Sets the cinematic configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetCinematicConfiguration {
    pub mask: u8,
    pub screen_size: u8,
    pub screen_distance: u8,
    pub ipd: u8,
    pub reserved0: [u8; 6],
    pub brightness: u8,
    pub mic_volume: u8,
    pub reserved1: [u8; 2],
    pub unknown: bool,
    pub reserved2: u8,
}

/// Sets the state of a LED on the HMD.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetHmdLeds {
    pub led_mask: u16,
    pub values: [u8; 9],
    pub reserved: [u8; 5],
}

/// Reads defice information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadDeviceInfo;

impl Command for SetPower {
    const ID: u8 = 0x17;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u32::<ByteOrder>(if self.on { 1 } else { 0 })
    }
}

impl Command for EnableVrTracking {
    const ID: u8 = 0x11;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u32::<ByteOrder>(0xFFFFFF00)?;
        write.write_u32::<ByteOrder>(0x00000000)?;
        Ok(())
    }
}

impl Command for SetVrMode {
    const ID: u8 = 0x23;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u32::<ByteOrder>(if self.vr_mode { 1 } else { 0 })
    }
}

impl Command for BoxOff {
    const ID: u8 = 0x13;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u32::<ByteOrder>(1)
    }
}

impl Command for SetCinematicConfiguration {
    const ID: u8 = 0x21;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u8(self.mask)?;
        write.write_u8(self.screen_size)?;
        write.write_u8(self.screen_distance)?;
        write.write_u8(self.ipd)?;
        write.write_all(&self.reserved0)?;
        write.write_u8(self.brightness)?;
        write.write_u8(self.mic_volume)?;
        write.write_all(&self.reserved1)?;
        write.write_u8(if self.unknown { 1 } else { 0 })?;
        write.write_u8(self.reserved2)
    }
}

impl Command for SetHmdLeds {
    const ID: u8 = 0x15;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u16::<ByteOrder>(self.led_mask)?;
        write.write_all(&self.values)?;
        write.write_all(&self.reserved)
    }
}

impl Command for ReadDeviceInfo {
    const ID: u8 = 0x81;

    fn write_payload(&self, write: &mut dyn Write) -> io::Result<()> {
        let reserved: [u8; 7] = [0; 7];
        write.write_u8(0x80)?;
        write.write_all(&reserved)
    }
}

#[cfg(test)]
mod invariants {
    use super::*;

    #[test]
    fn set_cinematic_configuration() {
        let c = SetCinematicConfiguration {
            mask: 0,
            screen_size: 0,
            screen_distance: 0,
            ipd: 0,
            reserved0: [0; 6],
            brightness: 77,
            mic_volume: 95,
            reserved1: [9, 9],
            unknown: true,
            reserved2: 127,
        };

        assert_eq!(16, c.payload_bytes().len());
    }

    #[test]
    fn set_hmd_leds() {
        let c = SetHmdLeds {
            led_mask: 0xdead,
            values: [4; 9],
            reserved: [5; 5],
        };

        assert_eq!(16, c.payload_bytes().len());
    }
}

