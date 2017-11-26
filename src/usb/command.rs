use std::io::prelude::*;
use std::io;

use usb::ByteOrder;
use byteorder::{WriteBytesExt};

/// A command that can be sent to the PSVR.
pub trait Command {
    const ID: u8;

    fn write_payload(&self, write: &mut Write) -> io::Result<()>;

    /// Gets the raw bytes that make up the payload.
    fn payload_bytes(&self) -> Vec<u8> {
        let mut buffer = io::Cursor::new(Vec::new());

        self.write_payload(&mut buffer).expect("encountered IO error while writing to memory");
        buffer.into_inner()
    }
}

/// Tells the PSVR to turn power off or on.
pub struct SetPower {
    /// `1` for on, `0` for off.
    pub on: u32,
}

/// Enables VR mode.
// NOTE: This command can probably be generalised to a 'set mode' command somehow.
pub struct EnableVrMode;

impl SetPower {
    /// Creates a new set power command.
    pub fn new(on: bool) -> Self {
        SetPower { on: if on { 1 } else { 0 } }
    }
}

impl Command for SetPower {
    const ID: u8 = 0x17;

    fn write_payload(&self, write: &mut Write) -> io::Result<()> {
        write.write_u32::<ByteOrder>(self.on)
    }
}

impl Command for EnableVrMode {
    const ID: u8 = 0x11;

    fn write_payload(&self, write: &mut Write) -> io::Result<()> {
        write.write_u32::<ByteOrder>(0xFFFFFF00)?;
        write.write_u32::<ByteOrder>(0x00000000)?;
        Ok(())
    }
}

