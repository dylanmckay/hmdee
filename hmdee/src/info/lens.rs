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
    pub chromatic_aberration: ChromaticAberration,
}

/// Chromatic aberration factors.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChromaticAberration {
    pub red: ChromaticAberrationFactor,
    pub green: ChromaticAberrationFactor,
    pub blue: ChromaticAberrationFactor,
}

/// A chromatic aberration factor.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChromaticAberrationFactor {
    /// The vertical aberration factor.
    pub vertical: math::Scalar,
    /// The horizontal aberration factor.
    pub horizontal: math::Scalar,
}

