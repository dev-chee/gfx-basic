use std::{f32, f64, fmt, iter, ops};

use super::Deg;

/// An angle, in radians.
#[derive(fmt::Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rad<T>(pub T);

impl_angle!(Rad<f32> { 0.0, f32::consts::FRAC_1_PI * 0.5, f32::consts::TAU, f32::consts::PI, f32::consts::FRAC_PI_2 });
impl_angle!(Rad<f64> { 0.0, f64::consts::FRAC_1_PI * 0.5, f64::consts::TAU, f64::consts::PI, f64::consts::FRAC_PI_2 });

macro_rules! impl_rad {
    ($T:ty) => {
        impl Rad<$T> {
            /// Compute the sine of the angle, returning a unitless ratio.
            pub fn sin(self) -> $T {
                self.0.sin()
            }

            /// Compute the cosine of the angle, returning a unitless ratio.
            pub fn cos(self) -> $T {
                self.0.cos()
            }

            /// Compute the tangent of the angle, returning a unitless ratio.
            pub fn tan(self) -> $T {
                self.0.tan()
            }

            /// Compute the sine and cosine of the angle, returning the result as a pair.
            pub fn sin_cos(self) -> ($T, $T) {
                self.0.sin_cos()
            }

            /// Compute the cosecant of the angle.
            pub fn csc(self) -> $T {
                self.sin().recip()
            }

            /// Compute the secant of the angle.
            pub fn sec(self) -> $T {
                self.cos().recip()
            }

            /// Compute the cotangent of the angle.
            pub fn cot(self) -> $T {
                self.tan().recip()
            }

            /// Compute the arcsine of the ratio, returning the resulting angle.
            pub fn asin(a: $T) -> Self {
                Self(a.asin())
            }

            /// Compute the arccosine of the ratio, returning the resulting angle.
            pub fn acos(a: $T) -> Self {
                Self(a.acos())
            }

            /// Compute the arctangent of the ratio, returning the resulting angle.
            pub fn atan(a: $T) -> Self {
                Self(a.atan())
            }

            /// Computes the arctangent of the a / b, returning the resulting angle.
            pub fn atan2(a: $T, b: $T) -> Self {
                Self(a.atan2(b))
            }
        }

        impl From<Deg<$T>> for Rad<$T> {
            fn from(value: Deg<$T>) -> Self {
                Self(value.0 * Self::HALF_TURN.0 / Deg::<$T>::HALF_TURN.0)
            }
        }
    };
}

impl_rad!(f32);
impl_rad!(f64);
