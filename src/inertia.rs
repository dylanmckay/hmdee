//! Inertia sensor handling (gyroscope, accelerometer).
//!
//! The PSVR uses a BMI055 Inertial Measurement Unit (IMU).
//!
//! The datasheet can be found [here](https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BMI055-DS000-08.pdf).
//!
//! BMI055 Features:
//!
//! * 6 degrees of freedom
//!   * Sensorscope
//!   * Accelerometer
//!   * There is no magnetometer. If there was, the PSVR would have 9 degrees of freedom

use math::{Quaternion, Scalar, Vector3};
use ahrs::{self, Ahrs};
use delta;

/// How many samples are taken per second.
const SAMPLE_FREQUENCY: u32 = 120;
/// How many seconds inbetween samples.
const SAMPLE_PERIOD: f32 = 1.0 / SAMPLE_FREQUENCY as f32;
/// The Madgwick beta constant for the PSVR.
const MADGWICK_BETA: f32 = 0.025;

/// Inertia information at a point in time.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Instant {
    /// The gyroscope inertia inforamation.
    pub gyroscope: Vector3,
    /// The accelerometer inertia inforamation.
    pub accelerometer: Vector3,
}

/// An inertia sensor.
#[derive(Debug)]
pub struct Sensor {
    /// The Madgwick AHRS implementation.
    madgwick: ahrs::Madgwick<Scalar>,
    /// The timer.
    timer: delta::Timer,
}

impl Sensor {
    /// Creates a new inertia sensor.
    pub fn new() -> Self {
        Sensor {
            madgwick: ahrs::Madgwick::new(SAMPLE_PERIOD, MADGWICK_BETA),
            timer: delta::Timer::new(),
        }
    }

    /// Updates the inertia sensor.
    pub fn update(&mut self, instant: &Instant) {
        let delta = self.timer.mark();

        // Change the sample period of the existing Madgwick object.
        // The library doesn't directly support dynamic periods.
        self.madgwick = ahrs::Madgwick::new_with_quat(delta as _, MADGWICK_BETA, self.madgwick.quat);

        self.madgwick.update_imu(
            &instant.gyroscope.into(),
            &instant.accelerometer.into(),
        ).expect("failed to run AHRS algorithm");
    }

    /// Gets the current orientation of the PSVR headset.
    pub fn hmd_orientation(&self) -> Quaternion {
        self.madgwick.quat
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_sensor() {
        let _ = Sensor::new();
    }
}

