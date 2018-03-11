//! hmdee VR library.
//!
//! * PSVR

pub use self::backend::HeadMountedDevice;
pub use self::context::Context;
pub use self::headset::Headset;
pub use self::discover::headsets;

// Show reexported crates like normal modules in Rustdoc.
pub use self::reexports::{core};
mod reexports {
    pub extern crate hmdee_core as core;
}

extern crate hidapi;
#[cfg(feature = "psvr")] extern crate psvr;

pub mod backend;
mod context;
mod headset;
mod discover;
pub mod input;
