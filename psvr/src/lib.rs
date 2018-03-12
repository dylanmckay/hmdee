extern crate hmdee_core;

extern crate ahrs;
extern crate byteorder;
extern crate delta;
pub extern crate hidapi;
extern crate nalgebra as na;

pub use self::client::*;

mod client;
pub mod command;
pub mod inertia;
pub mod protocol;
pub mod sensor;
mod usb;


