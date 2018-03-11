use core::math;

/// A head mounted device.
pub trait HeadMountedDevice {
    /// Gets the product name of the HMD.
    fn product_name(&self) -> &'static str;

    /// Gets the orientation of the headset.
    fn orientation(&self) -> math::Quaternion;
}

