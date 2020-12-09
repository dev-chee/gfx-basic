use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vector1, Vector2, Vector3};

/// A 4-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector4<T>
where
    T: fmt::Debug,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl_vector!(Vector4<f32>, 4, { x:0, y:1, z:2, w:3 }, 0.0, 1.0);
impl_vector!(Vector4<f64>, 4, { x:0, y:1, z:2, w:3 }, 0.0, 1.0);

macro_rules! impl_vector4 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector4<$T> {
            pub const UNIT_X: Self = Self {
                x: $one,
                y: $zero,
                z: $zero,
                w: $zero,
            };
            pub const UNIT_Y: Self = Self {
                x: $zero,
                y: $one,
                z: $zero,
                w: $zero,
            };
            pub const UNIT_Z: Self = Self {
                x: $zero,
                y: $zero,
                z: $one,
                w: $zero,
            };
            pub const UNIT_W: Self = Self {
                x: $zero,
                y: $zero,
                z: $zero,
                w: $one,
            };
        }

        impl From<($T, $T, $T, $T)> for Vector4<$T> {
            fn from(value: ($T, $T, $T, $T)) -> Self {
                Self::new(value.0, value.1, value.2, value.3)
            }
        }

        impl<'a> From<&'a ($T, $T, $T, $T)> for Vector4<$T> {
            fn from(value: &'a ($T, $T, $T, $T)) -> Self {
                Self::new(value.0, value.1, value.2, value.3)
            }
        }

        impl From<Vector1<$T>> for Vector4<$T> {
            fn from(v: Vector1<$T>) -> Self {
                Self::new(v.x, $zero, $zero, $zero)
            }
        }

        impl<'a> From<&'a Vector1<$T>> for Vector4<$T> {
            fn from(v: &'a Vector1<$T>) -> Self {
                Self::new(v.x, $zero, $zero, $zero)
            }
        }

        impl From<Vector2<$T>> for Vector4<$T> {
            fn from(v: Vector2<$T>) -> Self {
                Self::new(v.x, v.y, $zero, $zero)
            }
        }

        impl<'a> From<&'a Vector2<$T>> for Vector4<$T> {
            fn from(v: &'a Vector2<$T>) -> Self {
                Self::new(v.x, v.y, $zero, $zero)
            }
        }

        impl From<Vector3<$T>> for Vector4<$T> {
            fn from(v: Vector3<$T>) -> Self {
                Self::new(v.x, v.y, v.z, $zero)
            }
        }

        impl<'a> From<&'a Vector3<$T>> for Vector4<$T> {
            fn from(v: &'a Vector3<$T>) -> Self {
                Self::new(v.x, v.y, v.z, $zero)
            }
        }

        impl Into<($T, $T, $T, $T)> for Vector4<$T> {
            fn into(self) -> ($T, $T, $T, $T) {
                (self.x, self.y, self.z, self.w)
            }
        }
    };
}

impl_vector4!(f32, 1.0, 0.0);
impl_vector4!(f64, 1.0, 0.0);
