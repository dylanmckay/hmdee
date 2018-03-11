//! Backends for specific HMD devices.

#[cfg(feature = "psvr")] pub mod psvr;

/// A head mounted device.
pub trait HeadMountedDevice {
    /// Gets the product name of the HMD.
    fn product_name(&self) -> &'static str;
}

