use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vector2, Vector3, Vector4};

/// A 1-dimensional vector.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector1<T> {
    pub x: T,
}

impl_vector!(Vector1<f32>, { x:0; 1 }, { 0.0, 1.0 }, vec1_f32);
impl_vector1!(f32, 1.0, 0.0);

impl_vector!(Vector1<f64>, { x:0; 1 }, { 0.0, 1.0 }, vec1_f64);
impl_vector1!(f64, 1.0, 0.0);
