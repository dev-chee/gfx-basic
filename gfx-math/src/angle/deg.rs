use std::{f32, f64, fmt, iter, ops};

use super::Rad;

/// An angle, in degrees.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Deg<T>(pub T);

impl_angle!(Deg<f32> { 0.0, 1.0 / 360.0, 360.0, 180.0, 90.0 });
impl_deg!(f32);

impl_angle!(Deg<f64> { 0.0, 1.0 / 360.0, 360.0, 180.0, 90.0 });
impl_deg!(f64);
