#![allow(dead_code)]

use std::{fmt, ops};

use super::point::{Pt2, Pt3, Pt4};

#[macro_use]
mod macros;

define_vec!(Vec2, Pt2, f64, 1e-6, 2, { x:set_x:0, y:set_y:1 }, unit_x:[1.0, 0.0], unit_y:[0.0, 1.0]);
define_vec!(Vec3, Pt3, f64, 1e-6, 3, { x:set_x:0, y:set_y:1, z:set_z:2 }, unit_x:[1.0, 0.0, 0.0], unit_y:[0.0, 1.0, 0.0], unit_z:[0.0, 0.0, 1.0]);
define_vec!(Vec4, Pt4, f64, 1e-6, 4, { x:set_x:0, y:set_y:1, z:set_z:2, w:set_w:3 }, unit_x:[1.0, 0.0, 0.0, 0.0], unit_y:[0.0, 1.0, 0.0, 0.0], unit_z:[0.0, 0.0, 1.0, 0.0], unit_w:[0.0, 0.0, 0.0, 1.0]);

impl Vec3 {
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}

impl From<Vec3> for Vec2 {
    fn from(value: Vec3) -> Self {
        Self::new(value.x(), value.y())
    }
}

impl From<Vec4> for Vec2 {
    fn from(value: Vec4) -> Self {
        Self::new(value.x(), value.y())
    }
}

impl From<Vec2> for Vec3 {
    fn from(value: Vec2) -> Self {
        Self::new(value.x(), value.y(), 0.0)
    }
}

impl From<Vec4> for Vec3 {
    fn from(value: Vec4) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}

impl From<Vec2> for Vec4 {
    fn from(value: Vec2) -> Self {
        Self::new(value.x(), value.y(), 0.0, 0.0)
    }
}

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        Self::new(value.x(), value.y(), value.z(), 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        assert_eq!(Vec2::new(0.0, 0.0), Vec2::zero());
        assert_eq!(Vec2::new(1.0, 1.0), Vec2::from(1.0));
        assert_eq!(Vec2::new(1.0, 0.0), Vec2::unit_x());
        assert_eq!(Vec2::new(0.0, 1.0), Vec2::unit_y());
        assert_eq!(Vec2::from(1.0).x(), 1.0);
        assert_eq!(Vec2::from(1.0).y(), 1.0);
        assert_eq!(Vec2::zero().set_x(1.0).x(), 1.0);
        assert_eq!(Vec2::zero().set_y(1.0).y(), 1.0);

        assert_eq!(Vec3::new(0.0, 0.0, 0.0), Vec3::zero());
        assert_eq!(Vec3::new(1.0, 1.0, 1.0), Vec3::from(1.0));
        assert_eq!(Vec3::new(1.0, 0.0, 0.0), Vec3::unit_x());
        assert_eq!(Vec3::new(0.0, 1.0, 0.0), Vec3::unit_y());
        assert_eq!(Vec3::new(0.0, 0.0, 1.0), Vec3::unit_z());
        assert_eq!(Vec3::from(1.0).x(), 1.0);
        assert_eq!(Vec3::from(1.0).y(), 1.0);
        assert_eq!(Vec3::from(1.0).z(), 1.0);
        assert_eq!(Vec3::zero().set_x(1.0).x(), 1.0);
        assert_eq!(Vec3::zero().set_y(1.0).y(), 1.0);
        assert_eq!(Vec3::zero().set_z(1.0).z(), 1.0);

        assert_eq!(Vec4::new(0.0, 0.0, 0.0, 0.0), Vec4::zero());
        assert_eq!(Vec4::new(1.0, 1.0, 1.0, 1.0), Vec4::from(1.0));
        assert_eq!(Vec4::new(1.0, 0.0, 0.0, 0.0), Vec4::unit_x());
        assert_eq!(Vec4::new(0.0, 1.0, 0.0, 0.0), Vec4::unit_y());
        assert_eq!(Vec4::new(0.0, 0.0, 1.0, 0.0), Vec4::unit_z());
        assert_eq!(Vec4::new(0.0, 0.0, 0.0, 1.0), Vec4::unit_w());
        assert_eq!(Vec4::from(1.0).x(), 1.0);
        assert_eq!(Vec4::from(1.0).y(), 1.0);
        assert_eq!(Vec4::from(1.0).z(), 1.0);
        assert_eq!(Vec4::from(1.0).w(), 1.0);
        assert_eq!(Vec4::zero().set_x(1.0).x(), 1.0);
        assert_eq!(Vec4::zero().set_y(1.0).y(), 1.0);
        assert_eq!(Vec4::zero().set_z(1.0).z(), 1.0);
        assert_eq!(Vec4::zero().set_w(1.0).w(), 1.0);

        assert_eq!(
            <_ as Into<[_; 4]>>::into(Vec4::from(2.0)),
            [2.0, 2.0, 2.0, 2.0]
        );

        assert_eq!(Vec4::from([2.0, 2.0, 2.0, 2.0]), Vec4::from(2.0));
    }

    #[test]
    fn test_ops() {
        assert_eq!((Vec2::unit_x() + Vec2::unit_y()) * 2.0, Vec2::new(2.0, 2.0));
        assert_eq!(-(Vec2::unit_x() + Vec2::unit_y()), Vec2::new(-1.0, -1.0));

        let mut x = Vec2::unit_x();
        x += Vec2::unit_y();
        assert_eq!(x, Vec2::new(1.0, 1.0));

        let mut x = Vec2::unit_x();
        x -= Vec2::unit_y();
        assert_eq!(x, Vec2::new(1.0, -1.0));

        let mut x = Vec2::unit_x();
        x *= 2.0;
        assert_eq!(x, Vec2::new(2.0, 0.0));

        let mut x = Vec2::unit_x();
        x /= 2.0;
        assert_eq!(x, Vec2::new(0.5, 0.0));

        let mut x = Vec2::unit_x() * 7.0;
        x %= 2.0;
        assert_eq!(x, Vec2::new(1.0, 0.0));

        assert_eq!(
            (Vec3::unit_x() + Vec3::unit_y() + Vec3::unit_z()) * 2.0,
            Vec3::new(2.0, 2.0, 2.0)
        );
        assert_eq!(
            -(Vec3::unit_x() + Vec3::unit_y() + Vec3::unit_z()),
            Vec3::new(-1.0, -1.0, -1.0)
        );

        let mut x = Vec3::unit_x();
        x += Vec3::unit_y();
        x += Vec3::unit_z();
        assert_eq!(x, Vec3::new(1.0, 1.0, 1.0));

        let mut x = Vec3::unit_x();
        x -= Vec3::unit_y();
        x -= Vec3::unit_z();
        assert_eq!(x, Vec3::new(1.0, -1.0, -1.0));

        let mut x = Vec3::unit_x();
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, 0.0));

        let mut x = Vec3::unit_x();
        x /= 2.0;
        assert_eq!(x, Vec3::new(0.5, 0.0, 0.0));

        let mut x = Vec3::unit_x() * 7.0;
        x %= 2.0;
        assert_eq!(x, Vec3::new(1.0, 0.0, 0.0));

        assert_eq!(
            (Vec4::unit_x() + Vec4::unit_y() + Vec4::unit_z() + Vec4::unit_w()) * 2.0,
            Vec4::new(2.0, 2.0, 2.0, 2.0)
        );
        assert_eq!(
            -(Vec4::unit_x() + Vec4::unit_y() + Vec4::unit_z() + Vec4::unit_w()),
            Vec4::new(-1.0, -1.0, -1.0, -1.0)
        );

        let v = Vec4::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 4.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(v[3], 4.0);
    }

    #[test]
    fn test_dot_cross() {
        assert_eq!(Vec3::unit_x().cross(Vec3::unit_y()), Vec3::unit_z());
        assert_eq!(Vec3::unit_y().cross(Vec3::unit_x()), -Vec3::unit_z());

        assert_eq!(Vec3::unit_x().dot(&Vec3::unit_x()), 1.0);
        assert_eq!(Vec3::unit_x().dot(&Vec3::unit_y()), 0.0);
    }

    #[test]
    fn test_eq() {
        assert_ne!(Vec3::from(1.2), Vec3::from(1.0));
        assert_eq!(Vec3::from(1.2) * 0.7, Vec3::from(0.84));
    }
}
