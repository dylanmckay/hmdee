//! hmdee VR library.
//!
//! * PSVR

pub use self::backend::HeadMountedDevice;
pub use self::context::Context;
pub use self::headset::Headset;
pub use self::discover::headsets;

// Show reexported crates like normal modules in Rustdoc.
pub use self::reexports::{core};
pub use core::Error;
mod reexports {
    pub extern crate hmdee_core as core;
}

extern crate hidapi;

// Hide this here because we reexport it inside backend module.
#[cfg(feature = "psvr")] #[doc(hidden)] pub extern crate psvr;

pub mod backend;
mod context;
mod discover;
mod headset;
pub mod info;
pub mod input;
