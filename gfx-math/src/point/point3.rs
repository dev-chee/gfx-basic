use std::{fmt, ops};

use crate::vector::{Vector3, Vector4};

use super::{Point1, Point2};

/// A point in 3-dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point3<T>
where
    T: fmt::Debug,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl_point!(Vector3, Point3<f32>, 3, { x:0, y:1, z:2 }, 0.0, 1.0);
impl_point!(Vector3, Point3<f64>, 3, { x:0, y:1, z:2 }, 0.0, 1.0);

macro_rules! impl_point3 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Point3<$T> {
            pub fn from_homogeneous(value: Vector4<$T>) -> Self {
                let p = $one / value.w;
                Self::new(value.x * p, value.y * p, value.z * p)
            }

            pub fn to_homogeneous(self) -> Vector4<$T> {
                Vector4::<$T>::new(self.x, self.y, self.z, $zero)
            }
        }

        impl Into<($T, $T, $T)> for Point3<$T> {
            fn into(self) -> ($T, $T, $T) {
                (self.x, self.y, self.z)
            }
        }

        impl From<($T, $T, $T)> for Point3<$T> {
            fn from(value: ($T, $T, $T)) -> Self {
                Self::new(value.0, value.1, value.2)
            }
        }

        impl<'a> From<&'a ($T, $T, $T)> for Point3<$T> {
            fn from(value: &'a ($T, $T, $T)) -> Self {
                Self::new(value.0, value.1, value.2)
            }
        }

        impl From<Point1<$T>> for Point3<$T> {
            fn from(value: Point1<$T>) -> Self {
                Self::new(value.x, $zero, $zero)
            }
        }

        impl<'a> From<&'a Point1<$T>> for Point3<$T> {
            fn from(value: &'a Point1<$T>) -> Self {
                Self::new(value.x, $zero, $zero)
            }
        }

        impl From<Point2<$T>> for Point3<$T> {
            fn from(value: Point2<$T>) -> Self {
                Self::new(value.x, value.y, $zero)
            }
        }

        impl<'a> From<&'a Point2<$T>> for Point3<$T> {
            fn from(value: &'a Point2<$T>) -> Self {
                Self::new(value.x, value.y, $zero)
            }
        }
    };
}

impl_point3!(f32, 1.0, 0.0);
impl_point3!(f64, 1.0, 0.0);
