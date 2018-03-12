//! Information about a headset.

pub use self::lens::{ChromaticAberrationFactors, Lens};

mod lens;

use core::math;

/// Information about a VR headset.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Properties {
    /// Information about the headset's visuals.
    pub visuals: Visuals,
    /// How does the display of this.
    pub display_connector: DisplayConnector,
}

/// Information about the visuals
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Visuals {
    /// A headset with one or more lenses.
    ///
    /// Information about the left and right lenses are separate,
    /// so that devices with multiple different lens are supported.
    LensBased {
        /// The distance between the lens in micrometers.
        lens_separation: Distance,
        /// The baseline distance from the center of the lens to the eye.
        lens_to_eye_distance: Distance,
        /// The distance from the screen to the lens.
        screen_to_lens_distance: Distance,
        /// Information about the left lens.
        left: Lens,
        /// Information about the right lens.
        right: Lens,
    },
}

/// A connection method for a display.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DisplayConnector {
    /// The display connects via HDMI.
    Hdmi {
        /// The monitor name of the VR headset, as reported by HDMI EDID.
        monitor_name: String,
    },
}

/// A geometric distance.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distance { pub(crate) micrometers: u64 }

/// Information about a field of view.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FieldOfView {
    /// The horizontal field of view.
    pub horizontal: FieldOfViewAxis,
    /// The vertical field of view.
    pub vertical: FieldOfViewAxis,
}

/// Information about a field of view axis.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FieldOfViewAxis {
    /// The minimum field of view in degrees.
    pub minimum_degrees: math::Scalar,
    /// The maximum field of view in degrees.
    pub maximum_degrees: math::Scalar,
    /// The recommended field of view in degrees.
    pub recommended_degrees: math::Scalar,
}

impl Distance {
    /// Creates a new distance from a micrometer measurement.
    // TODO: make this a `const fn` when stable.
    pub fn from_micrometers(micrometers: u64) -> Self {
        Distance { micrometers }
    }

    /// Creates a new distance from a millimeter measurement.
    // TODO: make this a `const fn` when stable.
    pub fn from_millimeters(millimeters: u64) -> Self {
        Distance::from_micrometers(millimeters * 1_000)
    }

    /// Gets the distance in micrometers.
    pub fn micrometers(&self) -> u64 { self.micrometers }
    /// Gets the distance in millimeters.
    pub fn millimeters(&self) -> u64 { self.micrometers() / 1_000 }
}

