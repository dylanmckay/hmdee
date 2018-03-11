extern crate psvr;

use HeadMountedDevice;

use core::math;
use input;

/// A PlayStation VR headset.
pub struct Psvr<'hidapi> {
    /// The underlying PSVR structure.
    psvr: psvr::Psvr<'hidapi>,

    /// The latest readout from the PSVR sensors.
    latest_sensor_readout: Option<psvr::sensor::Readout>,
}

impl<'a> HeadMountedDevice for Psvr<'a> {
    fn product_name(&self) -> &'static str {
        "PlayStation VR"
    }

    fn orientation(&self) -> math::Quaternion {
        self.psvr.orientation()
    }

    fn button(&self, button: input::Button) -> input::ButtonState {
        match button {
            input::Button::VolumeUp => button_from_readout(&self.latest_sensor_readout, |r| r.buttons.plus),
            input::Button::VolumeDown => button_from_readout(&self.latest_sensor_readout, |r| r.buttons.minus),
            input::Button::Mute => button_from_readout(&self.latest_sensor_readout, |r| r.buttons.mute),
        }
    }
}

/// Gets the button state from a readout.
fn button_from_readout<F>(readout: &Option<psvr::sensor::Readout>, f: F) -> input::ButtonState
    where F: Fn(&psvr::sensor::Readout) -> bool {
    match readout.as_ref() {
        Some(readout) => if f(readout) { input::ButtonState::Pressed } else { input::ButtonState::NotPressed },
        None => input::ButtonState::NotPressed
    }
}

