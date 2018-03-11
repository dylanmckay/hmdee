//! Backends for specific HMD devices.

#[cfg(feature = "psvr")] pub mod psvr;

use {info, input};
use core::math;

/// A head mounted device.
pub trait HeadMountedDevice {
    /// Gets the product name of the HMD.
    fn product_name(&self) -> &'static str;

    /// Gets the orientation of the headset.
    fn orientation(&self) -> math::Quaternion;

    /// Gets the state of a button.
    fn button(&self, button: input::Button) -> input::ButtonState;

    /// Get information about the headset.
    fn properties(&self) -> &info::Properties;
}
