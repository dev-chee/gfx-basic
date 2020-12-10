#![allow(dead_code)]

mod vec2;
mod vec3;
mod vec4;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let epsilon = 1.0e-6;

        assert_eq!(Vec2::new(1.0, 0.0), Vec2::unit_x());
        assert_eq!(Vec2::from(1.0), Vec2::unit_x() + Vec2::unit_y());
        assert_eq!(
            Vec3::from(1.0),
            Vec3::unit_x() + Vec3::unit_y() + Vec3::unit_z()
        );
        assert_eq!(
            Vec4::from(1.0),
            Vec4::unit_x() + Vec4::unit_y() + Vec4::unit_z() + Vec4::unit_w()
        );

        assert!(Vec3::unit_x().dot(Vec3::unit_y()) - 0.0 < epsilon);
        assert!(Vec2::from(1.0).magnitude() - (2.0_f64).sqrt() < epsilon);
        assert!(Vec2::new(3.0, 4.0).normalize().magnitude() - 1.0 < epsilon);
        assert!(Vec2::unit_x()
            .project_on(Vec2::unit_y())
            .approx_eq(Vec2::zero(), epsilon.into()));
    }
}
