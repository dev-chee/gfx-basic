use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vector1, Vector2, Vector4};

/// A 3-dimensional vector.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl_vector!(Vector3<f32>, { x:0, y:1, z:2; 3 }, { 0.0, 1.0 }, vec3_f32);
impl_vector3!(f32, 1.0, 0.0);

impl_vector!(Vector3<f64>, { x:0, y:1, z:2; 3 }, { 0.0, 1.0 }, vec3_f64);
impl_vector3!(f64, 1.0, 0.0);
