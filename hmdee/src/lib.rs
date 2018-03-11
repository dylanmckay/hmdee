//! hmdee VR library.
//!
//! * PSVR

pub use self::hmd::HeadMountedDevice;

// Show reexported crates like normal modules in Rustdoc.
pub use self::reexports::{core};
mod reexports {
    pub extern crate hmdee_core as core;
}

pub mod backend;
mod hmd;
pub mod input;
