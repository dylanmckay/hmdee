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

use hmdee_core::math::{Quaternion, Scalar, Vector3};
use ahrs::{self, Ahrs};
use delta;

/// How many samples are taken per second.
const SAMPLE_FREQUENCY: u32 = 120;
/// How many seconds inbetween samples.
const SAMPLE_PERIOD: f32 = 1.0 / SAMPLE_FREQUENCY as f32;
/// The Madgwick beta constant for the PSVR.
const MADGWICK_BETA_ANTI_DRIFT: f32 = 0.125;
const MADGWICK_BETA_STEADINESS: Scalar = 0.035;

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
    integrators: Integrators,
    /// The timer.
    timer: delta::Timer,
}

#[derive(Debug)]
struct Integrators {
    /// A Madgwick filter optimized for its anti-drift qualities.
    anti_drift: ahrs::Madgwick<f32>,
    /// A Madgwick filter optimized for its steadying qualities.
    steadiness: ahrs::Madgwick<Scalar>,
}

impl Sensor {
    /// Creates a new inertia sensor.
    pub fn new() -> Self {
        Sensor {
            integrators: Integrators {
                anti_drift: ahrs::Madgwick::new(SAMPLE_PERIOD, MADGWICK_BETA_ANTI_DRIFT),
                steadiness: ahrs::Madgwick::new(SAMPLE_PERIOD, MADGWICK_BETA_STEADINESS),
            },
            timer: delta::Timer::new(),
        }
    }

    /// Updates the inertia sensor.
    pub fn update(&mut self, instant: &Instant) {
        let delta = self.timer.mark_secs();

        // Change the sample period of the existing Madgwick object.
        // The library doesn't directly support dynamic periods.
        self.integrators.anti_drift = ahrs::Madgwick::new_with_quat(delta as _, MADGWICK_BETA_ANTI_DRIFT, self.integrators.anti_drift.quat);
        self.integrators.steadiness = ahrs::Madgwick::new_with_quat(delta as _, MADGWICK_BETA_ANTI_DRIFT, self.integrators.steadiness.quat);

        self.integrators.anti_drift.update_imu(
            &instant.gyroscope.into(),
            &instant.accelerometer.into(),
        ).expect("failed to run anti-drift madgiwck filter");

        self.integrators.steadiness.update_imu(
            &instant.gyroscope.into(),
            &instant.accelerometer.into(),
        ).expect("failed to run steadiness madgiwck filter");
    }

    /// Gets the current orientation of the PSVR headset.
    pub fn hmd_orientation(&self) -> Quaternion {
        use na::geometry::UnitQuaternion;

        // FIXME: t parameter should be calculated on-the-fly.
        // https://github.com/dylanmckay/psvr-protocol/issues/14#issuecomment-435378326
        let t = 0.5;

        let anti_drift = UnitQuaternion::new_normalize(self.integrators.anti_drift.quat);
        let steadiness = UnitQuaternion::new_normalize(self.integrators.steadiness.quat);
        UnitQuaternion::slerp(&anti_drift, &steadiness, t).quaternion().clone()
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

