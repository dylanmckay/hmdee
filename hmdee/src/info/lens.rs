use info;
use core::math;

/// An individual lens.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Lens {
    /// The screen resolution of the lens.
    pub resolution: (u32, u32),
    /// The FOV of the lens.
    pub field_of_view: info::FieldOfView,
    /// The coefficients for the barrel distortion equation with this lens.
    ///
    /// The coefficients Ki correspond to the pincushion distortion equation:
    ///
    /// ```text
    /// p' = p (1 + K1 r^2 + K2 r^4 + ... + Kn r^(2n))
    /// ```
    pub distortion_coefficients: Vec<math::Scalar>,
    /// Chromatic aberration properties.
    pub chromatic_aberration_factors: ChromaticAberrationFactors,
}

/// Chromatic aberration factors.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChromaticAberrationFactors {
    pub red: math::Scalar,
    pub green: math::Scalar,
    pub blue: math::Scalar,
}

impl ChromaticAberrationFactors {
    /// Gets the factors corresponding to no aberration adjustment.
    pub fn no_adjustments() -> Self {
        ChromaticAberrationFactors {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
}

