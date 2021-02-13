//! Backends for specific HMD devices.

#[cfg(feature = "psvr")] mod backend_psvr;
#[cfg(feature = "psvr")] pub use self::backend_psvr::Psvr;
#[cfg(feature = "psvr")] pub use psvr;

use crate::{core::math, info, input, Error};

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

    /// Updates the headset state from a context.
    ///
    /// This should be called often.
    fn update(&mut self) -> Result<(), Error>;

    /// Powers on the headset.
    ///
    /// **Contract**: If the device is already on, nothing should happen.
    fn power_on(&mut self) -> Result<(), Error>;

    /// Powers off the headset.
    fn power_off(&mut self) -> Result<(), Error>;
}
