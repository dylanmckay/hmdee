//! Mathematics types.

use na;

/// The magnitude type.
pub type Scalar = f32;
/// The 2D vector type.
pub type Vector2 = na::Vector2<Scalar>;
/// The 3D vector type.
pub type Vector3 = na::Vector3<Scalar>;
/// The quaternion type.
pub type Quaternion = na::Quaternion<Scalar>;

