use core::math;
use {backend, input};

/// A head mounted device.
pub enum Headset<'context> {
    Psvr(backend::psvr::Psvr<'context>),
}

macro_rules! dispatch {
    { $self:expr => $method:ident ( $( $arg:expr ),* ) } => {
        match *$self {
            Headset::Psvr(ref psvr) => psvr . $method ( $( $arg ),* ),
        }
    }
}

impl<'context> backend::HeadMountedDevice for Headset<'context> {
    fn product_name(&self) -> &'static str {
        dispatch! { self => product_name() }
    }

    fn orientation(&self) -> math::Quaternion {
        dispatch! { self => orientation() }
    }

    fn button(&self, button: input::Button) -> input::ButtonState {
        dispatch! { self => button(button) }
    }
}

