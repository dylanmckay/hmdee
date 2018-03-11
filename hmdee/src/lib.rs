
// Show reexports like normal modules in Rustdoc.
pub use self::reexports::{core};
mod reexports {
    pub extern crate hmdee_core as core;
}

pub mod backend;

/// A head mounted device.
pub trait HeadMountedDevice {
    /// Gets the product name of the HMD.
    fn product_name(&self) -> &'static str;
}

