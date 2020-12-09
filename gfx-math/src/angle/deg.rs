use std::{f32, f64, fmt, iter, ops};

use super::Rad;

/// An angle, in degrees.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Deg<T>(pub T)
where
    T: fmt::Debug;

impl_angle!(Deg<f32> { 0.0, 1.0 / 360.0, 360.0, 180.0, 90.0 });
impl_angle!(Deg<f64> { 0.0, 1.0 / 360.0, 360.0, 180.0, 90.0 });

macro_rules! impl_deg {
    ($T:ty) => {
        impl Deg<$T> {
            /// Compute the sine of the angle, returning a unitless ratio.
            pub fn sin(self) -> $T {
                Rad::from(self).sin()
            }

            /// Compute the cosine of the angle, returning a unitless ratio.
            pub fn cos(self) -> $T {
                Rad::from(self).cos()
            }

            /// Compute the tangent of the angle, returning a unitless ratio.
            pub fn tan(self) -> $T {
                Rad::from(self).tan()
            }

            /// Compute the sine and cosine of the angle, returning the result as a pair.
            pub fn sin_cos(self) -> ($T, $T) {
                Rad::from(self).sin_cos()
            }

            /// Compute the cosecant of the angle.
            pub fn csc(self) -> $T {
                Rad::from(self).csc()
            }

            /// Compute the secant of the angle.
            pub fn sec(self) -> $T {
                Rad::from(self).sec()
            }

            /// Compute the cotangent of the angle.
            pub fn cot(self) -> $T {
                Rad::from(self).cot()
            }

            /// Compute the arcsine of the ratio, returning the resulting angle.
            pub fn asin(a: $T) -> Self {
                Rad::<$T>::asin(a).into()
            }

            /// Compute the arccosine of the ratio, returning the resulting angle.
            pub fn acos(a: $T) -> Self {
                Rad::<$T>::acos(a).into()
            }

            /// Compute the arctangent of the ratio, returning the resulting angle.
            pub fn atan(a: $T) -> Self {
                Rad::<$T>::atan(a).into()
            }

            /// Computes the arctangent of the a / b, returning the resulting angle.
            pub fn atan2(a: $T, b: $T) -> Self {
                Rad::<$T>::atan2(a, b).into()
            }
        }

        impl From<Rad<$T>> for Deg<$T> {
            fn from(rad: Rad<$T>) -> Self {
                Self(rad.0 * Self::HALF_TURN.0 / Rad::<$T>::HALF_TURN.0)
            }
        }
    };
}

impl_deg!(f32);
impl_deg!(f64);
