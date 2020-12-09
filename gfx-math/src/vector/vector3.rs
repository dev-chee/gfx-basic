use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vector1, Vector2, Vector4};

/// A 3-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<T>
where
    T: fmt::Debug,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl_vector!(Vector3<f32>, { x:0, y:1, z:2; 3 }, { 0.0, 1.0 });
impl_vector!(Vector3<f64>, { x:0, y:1, z:2; 3 }, { 0.0, 1.0 });

macro_rules! impl_vector3 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector3<$T> {
            pub const UNIT_X: Self = Self::new($one, $zero, $zero);
            pub const UNIT_Y: Self = Self::new($zero, $one, $zero);
            pub const UNIT_Z: Self = Self::new($zero, $zero, $one);

            /// Returns the cross product of the vector and `other`.
            pub fn cross(self, rhs: Self) -> Self {
                Self::new(
                    (self.y * rhs.z) - (self.z * rhs.y),
                    (self.z * rhs.x) - (self.x * rhs.z),
                    (self.x * rhs.y) - (self.y * rhs.x),
                )
            }
        }

        impl Into<($T, $T, $T)> for Vector3<$T> {
            fn into(self) -> ($T, $T, $T) {
                (self.x, self.y, self.z)
            }
        }

        impl From<($T, $T, $T)> for Vector3<$T> {
            fn from(value: ($T, $T, $T)) -> Self {
                Self::new(value.0, value.1, value.2)
            }
        }

        impl<'a> From<&'a ($T, $T, $T)> for Vector3<$T> {
            fn from(value: &'a ($T, $T, $T)) -> Self {
                Self::new(value.0, value.1, value.2)
            }
        }

        impl From<Vector1<$T>> for Vector3<$T> {
            fn from(v: Vector1<$T>) -> Self {
                Self::new(v.x, $zero, $zero)
            }
        }

        impl From<Vector2<$T>> for Vector3<$T> {
            fn from(v: Vector2<$T>) -> Self {
                Self::new(v.x, v.y, $zero)
            }
        }

        impl From<Vector4<$T>> for Vector3<$T> {
            fn from(v: Vector4<$T>) -> Self {
                Self::new(v.x, v.y, v.z)
            }
        }
    };
}

impl_vector3!(f32, 1.0, 0.0);
impl_vector3!(f64, 1.0, 0.0);
