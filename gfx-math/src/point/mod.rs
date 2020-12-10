#![allow(dead_code)]

mod pt2;
mod pt3;

pub use pt2::Pt2;
pub use pt3::Pt3;

#[cfg(test)]
mod tests {
    use crate::vector::Vec3;

    use super::*;

    #[test]
    fn test_point() {
        let epsilon = 1.0e-6;

        assert_eq!(
            Pt3::from(1.0),
            Pt3::origin() + Vec3::unit_x() + Vec3::unit_y() + Vec3::unit_z()
        );
        assert!((Pt3::new(1.0, 0.0, 0.0) - Pt3::origin()).dot(Vec3::unit_y()) - 0.0 < epsilon);
        assert!((Pt3::new(1.0, 0.0, 0.0) - Pt3::origin()).magnitude() - (2.0_f64).sqrt() < epsilon);
        assert!(
            <_ as Into<Vec3>>::into(Pt3::new(3.0, 4.0, 0.0))
                .normalize()
                .magnitude()
                - 1.0
                < epsilon
        );
        assert!((Pt3::new(1.0, 1.0, 0.0) - Pt3::origin())
            .project_on(Vec3::unit_y())
            .approx_eq(Vec3::unit_y(), epsilon.into()));
    }
}
