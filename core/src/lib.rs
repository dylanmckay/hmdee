//! The core HMD data types for all devices.

extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate nalgebra as na;

pub use self::error::Error;

mod error;
pub mod math;

