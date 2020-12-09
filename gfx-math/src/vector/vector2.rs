use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vector1, Vector3, Vector4};

/// A 2-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T>
where
    T: fmt::Debug,
{
    pub x: T,
    pub y: T,
}

impl_vector!(Vector2<f32>, { x:0, y:1; 2 }, { 0.0, 1.0 });
impl_vector!(Vector2<f64>, { x:0, y:1; 2 }, { 0.0, 1.0 });

macro_rules! impl_vector2 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector2<$T> {
            pub const UNIT_X: Self = Self::new($one, $zero);
            pub const UNIT_Y: Self = Self::new($zero, $one);

            /// The perpendicular dot product of the vector and `rhs`.
            pub fn perp_dot(self, rhs: Self) -> $T {
                (self.x * rhs.y) - (self.y * rhs.x)
            }
        }

        impl Into<($T, $T)> for Vector2<$T> {
            fn into(self) -> ($T, $T) {
                (self.x, self.y)
            }
        }

        impl From<($T, $T)> for Vector2<$T> {
            fn from(value: ($T, $T)) -> Self {
                Self::new(value.0, value.1)
            }
        }

        impl<'a> From<&'a ($T, $T)> for Vector2<$T> {
            fn from(value: &'a ($T, $T)) -> Self {
                Self::new(value.0, value.1)
            }
        }

        impl From<Vector1<$T>> for Vector2<$T> {
            fn from(v: Vector1<$T>) -> Self {
                Self::new(v.x, $zero)
            }
        }

        impl From<Vector3<$T>> for Vector2<$T> {
            fn from(v: Vector3<$T>) -> Self {
                Self::new(v.x, v.y)
            }
        }

        impl From<Vector4<$T>> for Vector2<$T> {
            fn from(v: Vector4<$T>) -> Self {
                Self::new(v.x, v.y)
            }
        }
    };
}

impl_vector2!(f32, 1.0, 0.0);
impl_vector2!(f64, 1.0, 0.0);
