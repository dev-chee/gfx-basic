use std::{f32, f64, fmt, iter, ops};

use super::Deg;

/// An angle, in radians.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rad<T>(pub T);

impl_angle!(Rad<f32> { 0.0, f32::consts::FRAC_1_PI * 0.5, f32::consts::TAU, f32::consts::PI, f32::consts::FRAC_PI_2 });
impl_rad!(f32);

impl_angle!(Rad<f64> { 0.0, f64::consts::FRAC_1_PI * 0.5, f64::consts::TAU, f64::consts::PI, f64::consts::FRAC_PI_2 });
impl_rad!(f64);
