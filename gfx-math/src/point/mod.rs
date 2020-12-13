#![allow(dead_code)]

use std::{fmt, ops};

use super::vector::{Vec2, Vec3, Vec4};

#[macro_use]
mod macros;

define_pt!(Pt2, Vec2, f64, 1e-6, 2, { x:set_x:0, y:set_y:1 });
define_pt!(Pt3, Vec3, f64, 1e-6, 3, { x:set_x:0, y:set_y:1, z:set_z:2 });
define_pt!(Pt4, Vec4, f64, 1e-6, 4, { x:set_x:0, y:set_y:1, z:set_z:2, w:set_w:3 });

impl fmt::Display for Pt2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl fmt::Display for Pt3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl From<Pt3> for Pt2 {
    fn from(value: Pt3) -> Self {
        Self::new(value.x(), value.y())
    }
}

impl From<Pt4> for Pt2 {
    fn from(value: Pt4) -> Self {
        Self::new(value.x(), value.y())
    }
}

impl From<Pt2> for Pt3 {
    fn from(value: Pt2) -> Self {
        Self::new(value.x(), value.y(), 0.0)
    }
}

impl From<Pt4> for Pt3 {
    fn from(value: Pt4) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}

impl From<Pt2> for Pt4 {
    fn from(value: Pt2) -> Self {
        Self::new(value.x(), value.y(), 0.0, 0.0)
    }
}

impl From<Pt3> for Pt4 {
    fn from(value: Pt3) -> Self {
        Self::new(value.x(), value.y(), value.z(), 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        assert_eq!(Pt2::new(0.0, 0.0), Pt2::origin());
        assert_eq!(Pt2::new(1.0, 1.0), Pt2::from(1.0));
        assert_eq!(Pt2::from(1.0).x(), 1.0);
        assert_eq!(Pt2::from(1.0).y(), 1.0);
        assert_eq!(Pt2::origin().set_x(1.0).x(), 1.0);
        assert_eq!(Pt2::origin().set_y(1.0).y(), 1.0);

        assert_eq!(Pt3::new(0.0, 0.0, 0.0), Pt3::origin());
        assert_eq!(Pt3::new(1.0, 1.0, 1.0), Pt3::from(1.0));
        assert_eq!(Pt3::from(1.0).x(), 1.0);
        assert_eq!(Pt3::from(1.0).y(), 1.0);
        assert_eq!(Pt3::from(1.0).z(), 1.0);
        assert_eq!(Pt3::origin().set_x(1.0).x(), 1.0);
        assert_eq!(Pt3::origin().set_y(1.0).y(), 1.0);
        assert_eq!(Pt3::origin().set_z(1.0).z(), 1.0);

        assert_eq!(<_ as Into<[_; 3]>>::into(Pt3::from(2.0)), [2.0, 2.0, 2.0]);

        assert_eq!(Pt3::from([2.0, 2.0, 2.0]), Pt3::from(2.0));
    }

    #[test]
    fn test_ops() {
        assert_eq!((Pt2::new(1.0, 0.0) + Vec2::unit_y()), Pt2::new(1.0, 1.0));
        assert_eq!(-(Vec2::unit_x() + Pt2::new(0.0, 1.0)), Pt2::new(-1.0, -1.0));

        let mut x = Pt2::new(1.0, 0.0);
        x += Vec2::unit_y();
        assert_eq!(x, Pt2::new(1.0, 1.0));

        let mut x = Pt2::new(1.0, 0.0);
        x -= Vec2::unit_y();
        assert_eq!(x, Pt2::new(1.0, -1.0));

        assert_eq!(
            (Pt3::new(1.0, 0.0, 0.0) + Vec3::unit_y() + Vec3::unit_z()),
            Pt3::from(1.0)
        );
        assert_eq!(
            -(Pt3::new(1.0, 0.0, 0.0) + Vec3::unit_y() + Vec3::unit_z()),
            Pt3::from(-1.0)
        );

        let mut x = Pt3::new(1.0, 0.0, 0.0);
        x += Vec3::unit_y();
        x += Vec3::unit_z();
        assert_eq!(x, Pt3::new(1.0, 1.0, 1.0));

        let mut x = Pt3::new(1.0, 0.0, 0.0);
        x -= Vec3::unit_y();
        x -= Vec3::unit_z();
        assert_eq!(x, Pt3::new(1.0, -1.0, -1.0));

        let v = Pt3::from([1.0, 2.0, 3.0]);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }
}
