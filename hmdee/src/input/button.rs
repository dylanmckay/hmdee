/// A button.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub enum Button {
    /// The volume up button.
    VolumeUp,
    /// The volume down button.
    VolumeDown,
    /// The mute button.
    Mute,
}

/// The state of an individual button.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub enum ButtonState {
    /// The button is up.
    NotPressed,
    /// The button is down.
    Pressed,
    /// The button is not present on this particular HMD.
    NotPresent,
}

impl ButtonState {
    /// Checks if the button is pressed.
    pub fn is_pressed(&self) -> bool {
        if let ButtonState::Pressed = *self { true } else { false }
    }

    /// Checks if this button is available.
    pub fn is_available(&self) -> bool {
        if let ButtonState::NotPresent = *self { false } else { true }
    }
}

