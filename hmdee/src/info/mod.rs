//! Information about a headset.

pub use self::lens::{ChromaticAberrationFactors, Lens};

mod lens;

use crate::core::math;

/// Information about a VR headset.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Properties {
    /// Information about the headset's visuals.
    pub visuals: Visuals,
    /// How does the display of this.
    pub display_info: DisplayInfo,
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

/// Information about a HMD's display.
///
/// # Finding the HDMI/VGA port of the HMD
///
/// The `DisplayInfo` type implements `Eq`. Any two `DisplayInfo`s can be
/// compared with one another for equality.
///
/// Two display information structures will be considered the same
/// if and only if they very similar to one another.
///
/// Use OS or library calls to enumerate the list of physical displays. Create
/// a `DisplayInfo` structure out of this. You can then find what
/// physical display represents the HMD screen by comparing the OS
/// `DisplayInfo` with the HMD's `DisplayInfo` returned by this library.
///
#[derive(Clone, Debug, PartialOrd)]
pub struct DisplayInfo {
    /// The monitor name of the VR headset, as reported by HDMI EDID.
    ///
    /// This is not necessarily 100% reliable for 1:1 matching against.
    /// Custom display profiles on each OS can be created by the user,
    /// possible assigning a different name, usually just adding a suffix
    /// to the original monitor name. Therefore, it is recommended to
    /// match monitor names by checking if the actual monitor name includes
    /// the HMD's monitor name as a substring.
    pub monitor_name: String,
    /// The physical size, in millimeters, of the HMD display if known.
    pub physical_size_millimeters: Option<(u32, u32)>,
    /// A list of `(width, height)` resolutions the headset is expected
    /// to support.
    ///
    /// This is not necessarily an exhaustive list, but every resolution
    /// in the list is guaranteed to be a resolution supported by the
    /// device.
    ///
    /// This can be used in HDMI input detection, for determining which
    /// HDMI port drives the HMD's screens.
    pub supported_resolutions: Vec<(u32, u32)>,
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

impl DisplayInfo {
    /// Finds the physical monitor handle that represents the HMD's display.
    ///
    /// This can be used to find which connected physical monitor on a
    /// system is the HMD.
    ///
    /// Returns `Some(physical monitor)` if the HMD is connected.
    ///
    /// Types:
    ///
    ///   * `P` - the physical monitor handle type
    ///   * `I` - an iterator of physical monitors returned by the OS
    ///   * `F` - a function that builds a `DisplayInfo` for a physical monitor.
    pub fn find_physical_monitor<P,I,F>(&self,
                                        monitors: I,
                                        f: F)
        -> Option<P>
        where P: Sized,
              I: IntoIterator<Item=P>,
              F: Fn(&P) -> DisplayInfo {
        monitors.into_iter().find(|monitor| f(monitor) == *self)
    }
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

impl PartialEq for DisplayInfo {
    fn eq(&self, other: &Self) -> bool {
        // Must perform rejection before acceptance.

        // Rejection crtieria start.
        {
            match (self.physical_size_millimeters, other.physical_size_millimeters) {
                // Displays that are different physical sizes cannot be the same.
                (Some(a), Some(b)) => if a != b { return false },
                _ => (),
            }

            // If neither supported resolution list subsumes the other, then
            // the displays cannot be the same.
            if !is_subset(&self.supported_resolutions, &other.supported_resolutions) &&
                !is_subset(&other.supported_resolutions, &self.supported_resolutions) {
                return false;
            }
        } // Rejection criteria end

        // If we got this far, it's a match.
        true
    }
}

fn is_subset(a: &[(u32, u32)], subset_of: &[(u32, u32)]) -> bool {
    a.iter().all(|t| subset_of.contains(t))
}


impl Eq for DisplayInfo { }
