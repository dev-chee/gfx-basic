use std::{fmt, ops};

use crate::vector::Vector1;

use super::{Point2, Point3};

/// A point in 1-dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point1<T>
where
    T: fmt::Debug,
{
    pub x: T,
}

impl_point!(Vector1, Point1<f32>, 1, { x:0 }, 0.0, 1.0);
impl_point!(Vector1, Point1<f64>, 1, { x:0 }, 0.0, 1.0);

macro_rules! impl_point1 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Into<($T,)> for Point1<$T> {
            fn into(self) -> ($T,) {
                (self.x,)
            }
        }

        impl From<($T,)> for Point1<$T> {
            fn from(value: ($T,)) -> Self {
                Self::new(value.0)
            }
        }

        impl<'a> From<&'a ($T,)> for Point1<$T> {
            fn from(value: &'a ($T,)) -> Self {
                Self::new(value.0)
            }
        }

        impl From<Point2<$T>> for Point1<$T> {
            fn from(value: Point2<$T>) -> Self {
                Self::new(value.x)
            }
        }

        impl<'a> From<&'a Point2<$T>> for Point1<$T> {
            fn from(value: &'a Point2<$T>) -> Self {
                Self::new(value.x)
            }
        }

        impl From<Point3<$T>> for Point1<$T> {
            fn from(value: Point3<$T>) -> Self {
                Self::new(value.x)
            }
        }

        impl<'a> From<&'a Point3<$T>> for Point1<$T> {
            fn from(value: &'a Point3<$T>) -> Self {
                Self::new(value.x)
            }
        }
    };
}

impl_point1!(f32, 1.0, 0.0);
impl_point1!(f64, 1.0, 0.0);
