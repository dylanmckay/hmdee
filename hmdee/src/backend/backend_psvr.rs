use Error;
use backend::HeadMountedDevice;

use core::math;
use {info, input};
use psvr;

const PSVR_HDMI_MONITOR_NAME: &'static str = "SIE  HMD *08";

const HMD_RESOLUTION_HORIZONTAL: u32 = 1920;
const HMD_RESOLUTION_VERTICAL: u32 = 1080;

const LENS_RESOLUTION_HORIZONTAL: u32 = HMD_RESOLUTION_HORIZONTAL / 2; // one lens for left, one for right.
const LENS_RESOLUTION_VERTICAL: u32 = HMD_RESOLUTION_VERTICAL; // both lens cover the full height of the display.

fn psvr_properties() -> info::Properties {

    let lens = info::Lens {
        resolution: (LENS_RESOLUTION_HORIZONTAL, LENS_RESOLUTION_VERTICAL),
        field_of_view: info::FieldOfView {
            horizontal: info::FieldOfViewAxis {
                minimum_degrees: 100.0,
                maximum_degrees: 100.0,
                recommended_degrees: 100.0,
            },
            vertical: info::FieldOfViewAxis {
                minimum_degrees: 68.0,
                maximum_degrees: 68.0,
                recommended_degrees: 68.0,
            },
        },
        distortion_coefficients: vec![0.247, -0.145, 0.103, 0.795],
        chromatic_aberration_factors: info::ChromaticAberrationFactors::no_adjustments(),
    };

    let visuals = info::Visuals::LensBased {
        left: lens.clone(), right: lens,
        lens_separation: info::Distance {
            micrometers: 63_100, // 63.1 millimeters.
        },
        lens_to_eye_distance: info::Distance {
            micrometers: 39_480, // 39.48 millimeters.
        },
        screen_to_lens_distance: info::Distance {
            micrometers: 35_400, // 35.4 millimeters
        },
    };

    info::Properties {
        display_info: info::DisplayInfo {
            monitor_name: PSVR_HDMI_MONITOR_NAME.to_owned(),
            physical_size_millimeters: Some((160, 90)),
            supported_resolutions: vec![
                (HMD_RESOLUTION_HORIZONTAL, HMD_RESOLUTION_VERTICAL),
            ],
        },
        visuals
    }
}

/// A PlayStation VR headset.
pub struct Psvr {
    /// The underlying PSVR structure.
    psvr: psvr::Psvr,

    /// The latest readout from the PSVR sensors.
    latest_sensor_readout: Option<psvr::sensor::Readout>,
    /// The headset properties.
    headset_properties: info::Properties,
}

impl Psvr {
    /// Gets the underlying PSVR client.
    pub fn underlying(&self) -> &psvr::Psvr { &self.psvr }
    /// Gets the underlying PSVR client.
    pub fn underlying_mut(&mut self) -> &mut psvr::Psvr { &mut self.psvr }
}

impl HeadMountedDevice for Psvr {
    fn product_name(&self) -> &'static str {
        "PlayStation VR"
    }

    fn orientation(&self) -> math::Quaternion {
        self.psvr.orientation()
    }

    fn button(&self, button: input::Button) -> input::ButtonState {
        match button {
            input::Button::VolumeUp => button_from_readout(&self.latest_sensor_readout, |r| r.buttons.plus),
            input::Button::VolumeDown => button_from_readout(&self.latest_sensor_readout, |r| r.buttons.minus),
            input::Button::Mute => button_from_readout(&self.latest_sensor_readout, |r| r.buttons.mute),
        }
    }

    fn properties(&self) -> &info::Properties {
        &self.headset_properties
    }

    fn update(&mut self) -> Result<(), Error> {
        let sensor_readout = self.psvr.receive_sensor()?;
        self.latest_sensor_readout = Some(sensor_readout);

        Ok(())
    }

    fn power_on(&mut self) -> Result<(), Error> {
        self.psvr.power_on()?;
        self.psvr.vr_mode()?;
        self.psvr.vr_tracking()?;

        Ok(())
    }

    fn power_off(&mut self) -> Result<(), Error> {
        self.psvr.power_off()
    }
}

impl From<psvr::Psvr> for Psvr {
    fn from(psvr: psvr::Psvr) -> Self {
        Psvr {
            latest_sensor_readout: None,
            psvr,
            headset_properties: psvr_properties(),
        }
    }
}

/// Gets the button state from a readout.
fn button_from_readout<F>(readout: &Option<psvr::sensor::Readout>, f: F) -> input::ButtonState
    where F: Fn(&psvr::sensor::Readout) -> bool {
    match readout.as_ref() {
        Some(readout) => if f(readout) { input::ButtonState::Pressed } else { input::ButtonState::NotPressed },
        None => input::ButtonState::NotPressed
    }
}

#[cfg(test)]
mod test {
    mod display_discovery {
        use super::super::psvr_properties;
        use info::*;

        // Display information from what the OS reports for the PSVR.
        fn example_psvr_physical_monitors() -> Vec<DisplayInfo> {
            vec![
                // How it appears on my MacOS 10.14 Mojave with
                // display modes patched to 4:4:4:4 chroma subsampling.
                DisplayInfo {
                    monitor_name: "SIE  HMD *08 (with EDID patch)".to_owned(),
                    physical_size_millimeters: Some((160, 90)),
                    // FIXME: complete the resolution list.
                    supported_resolutions: vec![
                        (1920, 1080),
                    ],
                },
            ]
        }

        #[test]
        fn matches_all_known_physical_monitors() {
            let properties = psvr_properties();

            for physical_monitor in example_psvr_physical_monitors() {
                assert_eq!(properties.display_info, physical_monitor);
            }
        }

        #[test]
        fn doesnt_match_when_both_physical_sizes_inconsistent() {
            let properties = psvr_properties();

            for mut physical_monitor in example_psvr_physical_monitors() {
                physical_monitor.physical_size_millimeters = Some((200, 200));

                assert!(properties.display_info != physical_monitor);
            }
        }
    }
}

