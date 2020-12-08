use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vector1, Vector3, Vector4};

/// A 2-dimensional vector.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl_vector!(Vector2<f32>, { x:0, y:1; 2 }, { 0.0, 1.0 }, vec2_f32);
impl_vector2!(f32, 1.0, 0.0);

impl_vector!(Vector2<f64>, { x:0, y:1; 2 }, { 0.0, 1.0 }, vec2_f64);
impl_vector2!(f64, 1.0, 0.0);
