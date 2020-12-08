#![allow(dead_code)]

#[macro_use]
mod macros;

mod vector1;
mod vector2;
mod vector3;
mod vector4;

pub use vector1::{vec1_f32, vec1_f64, Vector1};
pub use vector2::{vec2_f32, vec2_f64, Vector2};
pub use vector3::{vec3_f32, vec3_f64, Vector3};
pub use vector4::{vec4_f32, vec4_f64, Vector4};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        assert_eq!(vec1_f32(10.0), Vector1 { x: 10.0 });
    }

    #[test]
    fn test_index() {
        let v = vec3_f32(10.0, 20.0, 30.0);
        assert_eq!(v[0], v.x);
        assert_eq!(v[1], v.y);
    }
}
