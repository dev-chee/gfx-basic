#![allow(dead_code)]

#[macro_use]
mod macros;

mod point1;
mod point2;
mod point3;

pub use point1::Point1;
pub use point2::Point2;
pub use point3::Point3;

#[cfg(test)]
mod tests {
    type Pt3 = super::Point3<f64>;
    type Vec3 = crate::vector::Vector3<f64>;

    #[test]
    fn test_point() {
        let epsilon = 1.0e-6;

        assert_eq!(Pt3::from(1.0), Pt3::ORIGIN + Vec3::UNIT_X + Vec3::UNIT_Y + Vec3::UNIT_Z);
        assert!((Pt3::new(1.0, 0.0, 0.0) - Pt3::ORIGIN).dot(Vec3::UNIT_Y) - 0.0 < epsilon);
        assert!((Pt3::new(1.0, 0.0, 0.0) - Pt3::ORIGIN).magnitude() - (2.0_f64).sqrt() < epsilon);
        assert!(<_ as Into<Vec3>>::into(Pt3::new(3.0, 4.0, 0.0)).normalize().magnitude() - 1.0 < epsilon);
        assert!((Pt3::new(1.0, 1.0, 0.0) - Pt3::ORIGIN).project_on(Vec3::UNIT_Y).approx_eq(Vec3::UNIT_Y, epsilon.into()));
    }
}
