use core::math;
use input;

/// A head mounted device.
pub trait HeadMountedDevice {
    /// Gets the product name of the HMD.
    fn product_name(&self) -> &'static str;

    /// Gets the orientation of the headset.
    fn orientation(&self) -> math::Quaternion;

    /// Gets the state of a button.
    fn button(&self, button: input::Button) -> input::ButtonState;
}

