//! The sensor protocol.

use Error;
use usb::ByteOrder;

use std::io::prelude::*;
use std::{cmp, fmt, io};
use byteorder::ReadBytesExt;
use na;

pub type Scalar = f64;

/// The sensor frame size.
pub const FRAME_SIZE: usize = 64;

pub trait Readable : Sized {
    fn read(read: &mut Read) -> Result<Self, Error>;

    fn read_bytes(raw: &[u8; FRAME_SIZE]) -> Result<Self, Error> {
        Self::read(&mut io::Cursor::new(&raw[..]))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub buttons: Buttons,
    pub volume: u8,
    pub status: Status,
    pub instants: [Instant; 2],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Buttons {
    pub plus: bool,
    pub minus: bool,
    pub mute: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub worn: bool,
    pub display_active: bool,
    pub hdmi_disconnected: bool,
    pub microphone_muted: bool,
    pub headphone_connected: bool,
    pub tick: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Instant {
    /// The gyroscope readout.
    pub gyroscope_raw: na::Vector3<i16>,
    /// The accelerometer readout.
    pub accelerometer_raw: na::Vector3<i16>,
}

impl Instant {
    pub fn accelerometer(&self) -> na::Vector3<Scalar> {
        const RESOLUTION_BITS: u32 = 12;

        let f = |c| {
            let raw = c << 4; // Shift 12->16 bits.
            -(raw as f64 / 32768.0)
        };

        na::Vector3::new(f(self.accelerometer_raw.x),
                         f(self.accelerometer_raw.y),
                         f(-self.accelerometer_raw.z))
    }

    pub fn gyroscope(&self) -> na::Vector3<Scalar> {
        let f = |c| {
            (c as Scalar / 32768.0) * 2000.0
                * (::std::f64::consts::PI / 180.0) // DEGTORAD
        };

        na::Vector3::new(f(self.gyroscope_raw.x),
                         f(self.gyroscope_raw.y),
                         f(-self.gyroscope_raw.z))
    }
}

	// struct {
	//  	uint8_t reserved:1;
	//  	uint8_t plus:1;
	//  	uint8_t minus:1;
	//  	uint8_t mute:1;
	// } button;
	// uint8_t reserved0;
	// uint8_t volume;
	// uint8_t reserved1[5];
	// union {
	// 	uint8_t as_byte;
	// 	struct {
	// 		uint8_t worn:1;
	// 		uint8_t display_active:1;
	// 		uint8_t hdmi_disconnected:1;	// XXX
	// 		uint8_t microphone_muted:1;
	// 		uint8_t headphone_connected:1;
	// 		uint8_t reserved:2;
	// 		uint8_t tick:1;
	// 	};
	// } status;
	// uint8_t reserved2[11];
	// struct {
	// 	struct {
	// 		int16_t yaw;
	// 		int16_t pitch;
	// 		int16_t roll;
	// 	} gyro;
	// 	struct  {
	// 		int16_t x;
	// 		int16_t y;
	// 		int16_t z;
	// 	} accel;
	// 	uint8_t reserved[4];
	// } data[2];
	// uint8_t reserved3[12];

impl Readable for Frame {
    fn read(read: &mut Read) -> Result<Self, Error> {
        let buttons = Buttons::read(read)?;

        read_reserved(read, 1)?;
        let volume = read.read_u8()?;
        read_reserved(read, 5)?;
        let status = Status::read(read)?;
        read_reserved(read, 11)?;

        let instant_one = Instant::read(read)?;
        let instant_two = Instant::read(read)?;
        let instants = [instant_one, instant_two];
        read_reserved(read, 12)?;

        Ok(Frame {
            buttons, volume, status, instants,
        })
    }
}

/// Reads reserved data.
fn read_reserved(read: &mut Read, n: usize) -> Result<(), Error> {
    for _ in 0..n {
        read.read_u8()?;
    }
    Ok(())
}


impl Readable for Buttons {
    fn read(read: &mut Read) -> Result<Self, Error> {
        let b = read.read_u8()?;
        Ok(Buttons {
            // reserved:  (b & 0b0001) != 0,
            plus:  (b & 0b0010) != 0,
            minus: (b & 0b0100) != 0,
            mute:  (b & 0b1000) != 0,
        })
    }
}

impl Readable for Status {
    fn read(read: &mut Read) -> Result<Self, Error> {
        let b = read.read_u8()?;
        Ok(Status {
            worn:                (b & (1 << 0)) != 0,
            display_active:      (b & (1 << 1)) != 0,
            hdmi_disconnected:   (b & (1 << 2)) != 0,
            microphone_muted:    (b & (1 << 3)) != 0,
            headphone_connected: (b & (1 << 4)) != 0,
            // reserved:         (b & (1 << 5)) != 0,
            tick:                (b & (1 << 6)) != 0,
        })
    }
}

impl<T> Readable for na::Vector3<T>
    where T: Copy + Readable + fmt::Debug + cmp::PartialEq + 'static{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(na::Vector3::new(
            Readable::read(read)?,
            Readable::read(read)?,
            Readable::read(read)?,
        ))
    }
}

impl Readable for Instant {
    fn read(read: &mut Read) -> Result<Self, Error> {
        let gyroscope_raw = Readable::read(read)?;
        let accelerometer_raw = Readable::read(read)?;
        read_reserved(read, 4)?;

        Ok(Instant { gyroscope_raw, accelerometer_raw })
    }
}

macro_rules! impl_readable_primitive {
    ($ty:ident, $read_fn:ident, $byte_order:ty) => {
        impl Readable for $ty {
            fn read(read: &mut Read) -> Result<Self, Error> {
                Ok(read.$read_fn::<$byte_order>()?)
            }
        }
    };

    ($ty:ident, $read_fn:ident) => {
        impl Readable for $ty {
            fn read(read: &mut Read) -> Result<Self, Error> {
                Ok(read.$read_fn()?)
            }
        }
    };
}

impl_readable_primitive!(i8, read_i8);
impl_readable_primitive!(u8, read_u8);
impl_readable_primitive!(i16, read_i16, ByteOrder);
impl_readable_primitive!(u16, read_u16, ByteOrder);
impl_readable_primitive!(i32, read_i32, ByteOrder);
impl_readable_primitive!(u32, read_u32, ByteOrder);

#[cfg(test)]
mod test {
    use super::*;
    use std::io;

    #[test]
    fn reads_exactly_64_bytes() {
        let data: [u8; 64] = [0; 64];
        let mut read = io::Cursor::new(&data[..]);

        assert_eq!(0, read.position());
        Frame::read(&mut read).expect("failed to read frame");
        assert_eq!(FRAME_SIZE, read.position() as usize);
    }
}
