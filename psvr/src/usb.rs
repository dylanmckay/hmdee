//! PSVR usb stuff.

use hmdee_core;

/// The vendor ID of the PSVR.
pub const PSVR_VID: u16 = 0x054c;
/// The product ID of the PSVR.
pub const PSVR_PID: u16 = 0x09af;

/// The byte ordering used by the PSVR.
pub type ByteOrder = ::byteorder::LittleEndian;

/// PSVR USB interface definitions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interface {
    Audio3D = 0,
    AudioControl = 1,
    AudioMic = 2,
    AudioChat = 3,
    HidSensor = 4,
    HidControl = 5,
    // FIXME: Does 'VS_' start for Video Stream?
    VideoStreamH264 = 6,
    // FIXME: BulkIn is not a video stream, remove the prefix.
    VideoStreamBulkIn = 7,
    HidControl2 = 8,
}

impl Interface {
    pub fn from_i32(value: i32) -> Result<Self, hmdee_core::Error> {
        use usb::Interface::*;

        match value {
            0 => Ok(Audio3D),
            1 => Ok(AudioControl),
            2 => Ok(AudioMic),
            3 => Ok(AudioChat),
            4 => Ok(HidSensor),
            5 => Ok(HidControl),
            6 => Ok(VideoStreamH264),
            7 => Ok(VideoStreamBulkIn),
            8 => Ok(HidControl2),
            _ => Err(hmdee_core::Error::CommunicationError {
                message: format!("usb interface '{}' is not a known PSVR interface number", value).into()
            }),
        }
    }
}

