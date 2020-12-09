#![allow(dead_code)]

#[macro_use]
mod macros;

mod vector1;
mod vector2;
mod vector3;
mod vector4;

pub use vector1::Vector1;
pub use vector2::Vector2;
pub use vector3::Vector3;
pub use vector4::Vector4;

#[cfg(test)]
mod tests {
    type Rad = crate::angle::Rad::<f64>;
    type Vec1 = super::Vector1<f64>;
    type Vec2 = super::Vector2<f64>;
    type Vec3 = super::Vector3<f64>;
    type Vec4 = super::Vector4<f64>;

    #[test]
    fn test_vector() {
        let epsilon = 1.0e-6;

        assert_eq!(Vec1::from(1.0), Vec1::UNIT_X);
        assert_eq!(Vec2::from(1.0), Vec2::UNIT_X + Vec2::UNIT_Y);
        assert_eq!(Vec3::from(1.0), Vec3::UNIT_X + Vec3::UNIT_Y + Vec3::UNIT_Z);
        assert_eq!(
            Vec4::from(1.0),
            Vec4::UNIT_X + Vec4::UNIT_Y + Vec4::UNIT_Z + Vec4::UNIT_W
        );

        assert!(Vec3::UNIT_X.dot(Vec3::UNIT_Y) - 0.0 < epsilon);
        assert!(Vec2::from(1.0).magnitude() - (2.0_f64).sqrt() < epsilon);
        assert!(Vec2::new(3.0, 4.0).normalize().magnitude() - 1.0 < epsilon);
        assert!(Vec2::UNIT_X.project_on(Vec2::UNIT_Y).approx_eq(Vec2::ZERO, epsilon.into()));
    }
}
