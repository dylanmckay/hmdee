use Error;
use {backend, info, input};
use core::math;

/// A head mounted device.
pub enum Headset<'context> {
    Psvr(backend::Psvr),
    #[doc(hidden)]
    Phantom(std::marker::PhantomData<&'context ()>),
}

macro_rules! dispatch {
    { $self:expr => $method:ident ( $( $arg:expr ),* ) } => {
        match *$self {
            Headset::Psvr(ref psvr) => psvr . $method ( $( $arg ),* ),
            Headset::Phantom(..) => unreachable!(),
        }
    };

    { mut $self:expr => $method:ident ( $( $arg:expr ),* ) } => {
        match *$self {
            Headset::Psvr(ref mut psvr) => psvr . $method ( $( $arg ),* ),
            Headset::Phantom(..) => unreachable!(),
        }
    };
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

    fn properties(&self) -> &info::Properties {
        dispatch! { self => properties() }
    }

    fn update(&mut self) -> Result<(), Error> {
        dispatch! { mut self => update() }
    }

    fn power_on(&mut self) -> Result<(), Error> {
        dispatch! { mut self => power_on() }
    }

    fn power_off(&mut self) -> Result<(), Error> {
        dispatch! { mut self => power_off() }
    }
}

