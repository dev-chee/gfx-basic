use crate::vector::{Vec2, Vec3, Vec4};

#[macro_use]
mod macros;

define_mat!(Mat2x2, Vec2, 2);
define_mat!(Mat3x3, Vec3, 3);
define_mat!(Mat4x4, Vec4, 4);
