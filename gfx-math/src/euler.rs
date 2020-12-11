use angle::Deg;

use crate::angle;

/// A set of [Euler angles] representing a rotation in three-dimensional space.
///
/// The axis rotation sequence is XYZ. That is, the rotation is first around
/// the X axis, then the Y axis, and lastly the Z axis (using intrinsic
/// rotations). Since all three rotation axes are used, the angles are
/// Tait–Bryan angles rather than proper Euler angles.
///
/// # Ranges
///
/// - x: [-pi, pi]
/// - y: [-pi/2, pi/2]
/// - z: [-pi, pi]
///
/// # Defining rotations using Euler angles
///
/// Note that while [Euler angles] are intuitive to define, they are prone to
/// [gimbal lock] and are challenging to interpolate between. Instead we
/// recommend that you convert them to a more robust representation, such as a
/// quaternion or a rotation matrix. To this end, `From<Euler<A>>` conversions
/// are provided for the following types:
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Euler {
    /// The angle to apply around the _x_ axis. Also known at the _pitch_.
    pub x: Deg,
    /// The angle to apply around the _y_ axis. Also known at the _yaw_.
    pub y: Deg,
    /// The angle to apply around the _z_ axis. Also known at the _roll_.
    pub z: Deg,
}

impl Euler {
    /// Construct a set of euler angles.
    ///
    /// # Arguments
    ///
    /// * `x` - The angle to apply around the _x_ axis. Also known at the _pitch_.
    /// * `y` - The angle to apply around the _y_ axis. Also known at the _yaw_.
    /// * `z` - The angle to apply around the _z_ axis. Also known at the _roll_.
    pub const fn new(x: Deg, y: Deg, z: Deg) -> Self {
        Self { x, y, z }
    }
}
