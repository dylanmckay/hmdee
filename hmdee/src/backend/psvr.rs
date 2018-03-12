use backend::HeadMountedDevice;

use core::math;
use {info, input};
use psvr;

const PSVR_HDMI_MONITOR_NAME: &'static str = "SIE  HMD *08";

fn psvr_properties() -> info::Properties {
    const LENS_WIDTH: u32 = 1920 / 2;
    const DISPLAY_HEIGHT: u32 = 1080;

    let lens = info::Lens {
        resolution: (LENS_WIDTH, DISPLAY_HEIGHT),
        field_of_view: info::FieldOfView {
            horizontal: info::FieldOfViewAxis {
                minimum_degrees: 100.0,
                maximum_degrees: 100.0,
                recommended_degrees: 100.0,
            },
            // FIXME: confirm these numbers
            // I cannot find any information about the _vertical_
            // FOV of the PSVR.
            vertical: info::FieldOfViewAxis {
                minimum_degrees: 100.0,
                maximum_degrees: 100.0,
                recommended_degrees: 100.0,
            },
        },
        distortion_coefficients: vec![0.22, 0.24],
        chromatic_aberration: info::ChromaticAberration {
            red: info::ChromaticAberrationFactor {
                vertical: 1.0, horizontal: 1.0,
            },
            green: info::ChromaticAberrationFactor {
                vertical: 1.0078, horizontal: 1.0091,
            },
            blue: info::ChromaticAberrationFactor {
                vertical: 1.0192, horizontal: 1.0224,
            },
        },
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
        display_connector: info::DisplayConnector::Hdmi { monitor_name: PSVR_HDMI_MONITOR_NAME.to_owned() },
        visuals
    }
}

/// A PlayStation VR headset.
pub struct Psvr<'hidapi> {
    /// The underlying PSVR structure.
    psvr: psvr::Psvr<'hidapi>,

    /// The latest readout from the PSVR sensors.
    latest_sensor_readout: Option<psvr::sensor::Readout>,
    /// The headset properties.
    headset_properties: info::Properties,
}

impl<'a> HeadMountedDevice for Psvr<'a> {
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

    fn update(&mut self) {
        let sensor_readout = self.psvr.receive_sensor().unwrap();
        self.latest_sensor_readout = Some(sensor_readout);
    }

    fn power_on(&mut self) {
        self.psvr.power_on().unwrap();
        self.psvr.vr_mode().unwrap();
        self.psvr.vr_tracking().unwrap();
    }

    fn power_off(&mut self) {
        self.psvr.power_off().unwrap();
    }
}

impl<'context> From<psvr::Psvr<'context>> for Psvr<'context> {
    fn from(psvr: psvr::Psvr<'context>) -> Self {
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

