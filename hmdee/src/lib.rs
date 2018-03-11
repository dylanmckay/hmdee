
// Show reexports like normal modules in Rustdoc.
pub use self::reexports::{core};
mod reexports {
    pub extern crate hmdee_core as core;
}

pub mod backend;

