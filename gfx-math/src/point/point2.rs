use std::{fmt, ops};

use crate::vector::Vector2;

use super::{Point1, Point3};

/// A point in 2-dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point2<T>
where
    T: fmt::Debug,
{
    pub x: T,
    pub y: T,
}

impl_point!(Vector2, Point2<f32>, 2, { x:0, y:1 }, 0.0, 1.0);
impl_point!(Vector2, Point2<f64>, 2, { x:0, y:1 }, 0.0, 1.0);

macro_rules! impl_point2 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Into<($T, $T)> for Point2<$T> {
            fn into(self) -> ($T, $T) {
                (self.x, self.y)
            }
        }

        impl From<($T, $T)> for Point2<$T> {
            fn from(value: ($T, $T)) -> Self {
                Self::new(value.0, value.1)
            }
        }

        impl<'a> From<&'a ($T, $T)> for Point2<$T> {
            fn from(value: &'a ($T, $T)) -> Self {
                Self::new(value.0, value.1)
            }
        }

        impl From<Point1<$T>> for Point2<$T> {
            fn from(value: Point1<$T>) -> Self {
                Self::new(value.x, $zero)
            }
        }

        impl<'a> From<&'a Point1<$T>> for Point2<$T> {
            fn from(value: &'a Point1<$T>) -> Self {
                Self::new(value.x, $zero)
            }
        }

        impl From<Point3<$T>> for Point2<$T> {
            fn from(value: Point3<$T>) -> Self {
                Self::new(value.x, value.y)
            }
        }

        impl<'a> From<&'a Point3<$T>> for Point2<$T> {
            fn from(value: &'a Point3<$T>) -> Self {
                Self::new(value.x, value.y)
            }
        }
    };
}

impl_point2!(f32, 1.0, 0.0);
impl_point2!(f64, 1.0, 0.0);
