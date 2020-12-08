use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vector1, Vector2, Vector3};

/// A 4-dimensional vector.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl_vector!(Vector4<f32>, { x:0, y:1, z:2, w:3; 4 }, { 0.0, 1.0 }, vec4_f32);
impl_vector4!(f32, 1.0, 0.0);

impl_vector!(Vector4<f64>, { x:0, y:1, z:2, w:3; 4 }, { 0.0, 1.0 }, vec4_f64);
impl_vector4!(f64, 1.0, 0.0);
