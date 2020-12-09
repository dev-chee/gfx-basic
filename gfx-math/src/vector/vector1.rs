use std::{fmt, iter, ops};

use crate::{angle::Rad, point::Point1};

use super::{Vector2, Vector3, Vector4};

/// A 1-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector1<T>
where
    T: fmt::Debug,
{
    pub x: T,
}

impl_vector!(Vector1<f32>, 1, { x:0 }, 0.0, 1.0);
impl_vector!(Vector1<f64>, 1, { x:0 }, 0.0, 1.0);

macro_rules! impl_vector1 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector1<$T> {
            pub const UNIT_X: Self = Self { x: $one };
        }

        impl ops::Add<Point1<$T>> for Vector1<$T> {
            type Output = Point1<$T>;

            fn add(self, rhs: Point1<$T>) -> Point1<$T> {
                Point1::<$T>::new(self.x + rhs.x)
            }
        }

        impl<'a> ops::Add<&'a Point1<$T>> for Vector1<$T> {
            type Output = Point1<$T>;

            fn add(self, rhs: &'a Point1<$T>) -> Point1<$T> {
                Point1::<$T>::new(self.x + rhs.x)
            }
        }

        impl From<Point1<$T>> for Vector1<$T> {
            fn from(value: Point1<$T>) -> Self {
                value - Point1::<$T>::ORIGIN
            }
        }

        impl<'a> From<&'a Point1<$T>> for Vector1<$T> {
            fn from(value: &'a Point1<$T>) -> Self {
                *value - Point1::<$T>::ORIGIN
            }
        }

        impl From<($T,)> for Vector1<$T> {
            fn from(value: ($T,)) -> Self {
                Self::new(value.0)
            }
        }

        impl<'a> From<&'a ($T,)> for Vector1<$T> {
            fn from(value: &'a ($T,)) -> Self {
                Self::new(value.0)
            }
        }

        impl From<Vector2<$T>> for Vector1<$T> {
            fn from(v: Vector2<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl<'a> From<&'a Vector2<$T>> for Vector1<$T> {
            fn from(v: &'a Vector2<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl From<Vector3<$T>> for Vector1<$T> {
            fn from(v: Vector3<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl<'a> From<&'a Vector3<$T>> for Vector1<$T> {
            fn from(v: &'a Vector3<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl From<Vector4<$T>> for Vector1<$T> {
            fn from(v: Vector4<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl<'a> From<&'a Vector4<$T>> for Vector1<$T> {
            fn from(v: &'a Vector4<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl Into<($T,)> for Vector1<$T> {
            fn into(self) -> ($T,) {
                (self.x,)
            }
        }
    };
}

impl_vector1!(f32, 1.0, 0.0);
impl_vector1!(f64, 1.0, 0.0);
