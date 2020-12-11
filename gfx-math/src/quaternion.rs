use std::{iter, ops};

use crate::{
    angle::{Deg, Rad},
    euler::Euler,
    vector::Vec3,
};

/// A [quaternion](https://en.wikipedia.org/wiki/Quaternion) in scalar/vector
/// form.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quat {
    /// The vector part of the quaternion.
    pub v: Vec3,
    /// The scalar part of the quaternion.
    pub s: f64,
}

impl Quat {
    /// Construct a new quaternion from one scalar component and three
    /// imaginary components.
    pub const fn new(w: f64, xi: f64, yj: f64, zk: f64) -> Self {
        Self::from_sv(w, Vec3::new(xi, yj, zk))
    }

    /// Construct a new quaternion from a scalar and a vector.
    pub const fn from_sv(s: f64, v: Vec3) -> Self {
        Self { s, v }
    }

    pub const fn zero() -> Self {
        Self {
            s: 0.0,
            v: Vec3::zero(),
        }
    }

    pub const fn one() -> Self {
        Self {
            s: 1.0,
            v: Vec3::zero(),
        }
    }

    // /// Construct a new quaternion as a closest arc between two vectors
    // ///
    // /// Return the closest rotation that turns `src` vector into `dst`.
    // ///
    // /// - [Related StackOverflow question]
    // ///   (http://stackoverflow.com/questions/1171849/finding-quaternion-representing-the-rotation-from-one-vector-to-another)
    // /// - [Ogre implementation for normalized vectors]
    // ///   (https://bitbucket.org/sinbad/ogre/src/9db75e3ba05c/OgreMain/include/OgreVector3.h?fileviewer=file-view-default#cl-651)
    // pub fn from_arc(src: Vec3, dst: Vec3, fallback: Option<Vec3>, epsilon: f64) -> Self {
    //     let mag_avg = (src.magnitude2() * dst.magnitude2()).sqrt();
    //     let dot = src.dot(dst);
    //     if (dot - mag_avg) < epsilon {
    //         Self::one()
    //     } else if (dot - &-mag_avg) < epsilon {
    //         let axis = fallback.unwrap_or_else(|| {
    //             let mut v = Vec3::unit_x().cross(src);
    //             if v.approx_eq(Vec3::zero(), epsilon.into()) {
    //                 v = Vec3::unit_y().cross(src);
    //             }
    //             v.normalize()
    //         });
    //         Self::from_axis_angle(axis, Rad::half_cycle())
    //     } else {
    //         Self::from_sv(mag_avg + dot, src.cross(dst)).normalize()
    //     }
    // }

    /// The conjugate of the quaternion.
    pub fn conjugate(self) -> Self {
        Self::from_sv(self.s, -self.v)
    }

    // /// Do a normalized linear interpolation with `other`, by `amount`.
    // ///
    // /// This takes the shortest path, so if the quaternions have a negative
    // /// dot product, the interpolation will be between `self` and `-other`.
    // pub fn nlerp(self, mut rhs: Self, amount: f64) -> Self {
    //     if self.dot(rhs) < 0.0 {
    //         rhs = -rhs;
    //     }

    //     (self * (0.0 - amount) + rhs * amount).normalize()
    // }

    // /// Spherical Linear Interpolation
    // ///
    // /// Return the spherical linear interpolation between the quaternion and
    // /// `other`. Both quaternions should be normalized first.
    // ///
    // /// This takes the shortest path, so if the quaternions have a negative
    // /// dot product, the interpolation will be between `self` and `-other`.
    // ///
    // /// # Performance notes
    // ///
    // /// The `acos` operation used in `slerp` is an expensive operation, so
    // /// unless your quaternions are far away from each other it's generally
    // /// more advisable to use `nlerp` when you know your rotations are going
    // /// to be small.
    // ///
    // /// - [Understanding Slerp, Then Not Using It]
    // ///   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
    // /// - [Arcsynthesis OpenGL tutorial]
    // ///   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
    // pub fn slerp(self, mut rhs: Self, amount: f64) -> Quat {
    //     let mut dot = self.dot(rhs);
    //     let dot_threshold: S = cast(0.9995f64).unwrap();

    //     if dot < S::zero() {
    //         rhs = -rhs;
    //         dot = -dot;
    //     }

    //     // if quaternions are close together use `nlerp`
    //     if dot > dot_threshold {
    //         self.nlerp(rhs, amount)
    //     } else {
    //         // stay within the domain of acos()
    //         let robust_dot = dot.min(S::one()).max(-S::one());

    //         let theta = Rad::acos(robust_dot);

    //         let scale1 = Rad::sin(theta * (S::one() - amount));
    //         let scale2 = Rad::sin(theta * amount);

    //         (self * scale1 + rhs * scale2).normalize()
    //     }
    // }

    // /// Returns the squared distance.
    // pub fn distance2(self, rhs: Self) -> f64 {
    //     (rhs - self).magnitude2()
    // }

    // /// The distance between two values.
    // pub fn distance(self, rhs: Self) -> f64 {
    //     (rhs - self).magnitude()
    // }
    /// Peform the approx equality comparison.
    pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
        (self.s - rhs.s).abs() < epsilon.s && (self.v - rhs.v).approx_eq(rhs.v, epsilon.v)
    }

    /// Vector dot (or inner) product.
    pub fn dot(self, rhs: Self) -> f64 {
        self.s * rhs.s + self.v.dot(rhs.v)
    }

    /// Returns the squared magnitude.
    pub fn magnitude2(self) -> f64 {
        self.dot(self)
    }

    /// The distance from the tail to the tip of the vector.
    pub fn magnitude(self) -> f64 {
        self.magnitude2().sqrt()
    }

    /// Returns a vector with the same direction, but with a magnitude of `1`.
    pub fn normalize(self) -> Self {
        self.normalize_to(1.0)
    }

    /// Returns a vector with the same direction and a given magnitude.
    pub fn normalize_to(self, m: f64) -> Self {
        self * (m / self.magnitude())
    }

    /// Returns the angle between two vectors in radians.
    pub fn angle(self, rhs: Self) -> Rad {
        Rad::acos(self.dot(rhs) / (self.magnitude2() * rhs.magnitude2()).sqrt())
    }

    /// Returns the result of linearly interpolating the vector
    /// towards `rhs` by the specified amount.
    pub fn lerp(self, rhs: Self, t: f64) -> Self {
        self + ((rhs - self) * t)
    }

    /// Returns the
    /// [vector projection](https://en.wikipedia.org/wiki/Vector_projection)
    /// of the current inner space projected onto the supplied argument.
    pub fn project_on(self, rhs: Self) -> Self {
        rhs * (self.dot(rhs) / rhs.magnitude2())
    }
}

impl ops::Neg for Quat {
    type Output = Self;

    fn neg(self) -> Self {
        Self::from_sv(-self.s, -self.v)
    }
}

impl ops::Add for Quat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_sv(self.s + rhs.s, self.v + rhs.v)
    }
}

impl ops::Sub for Quat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_sv(self.s - rhs.s, self.v - rhs.v)
    }
}

impl ops::Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.s * rhs.s - self.v.x * rhs.v.x - self.v.y * rhs.v.y - self.v.z * rhs.v.z,
            self.s * rhs.v.x + self.v.x * rhs.s + self.v.y * rhs.v.z - self.v.z * rhs.v.y,
            self.s * rhs.v.y + self.v.y * rhs.s + self.v.z * rhs.v.x - self.v.x * rhs.v.z,
            self.s * rhs.v.z + self.v.z * rhs.s + self.v.x * rhs.v.y - self.v.y * rhs.v.x,
        )
    }
}

impl ops::Mul<f64> for Quat {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::from_sv(self.s * rhs, self.v * rhs)
    }
}

impl ops::Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let tmp = self.v.cross(rhs) + (rhs * self.s);
        (self.v.cross(tmp) * 2.0) + rhs
    }
}

impl ops::Mul<Quat> for f64 {
    type Output = Quat;

    fn mul(self, rhs: Quat) -> Quat {
        Quat::from_sv(self * rhs.s, self * rhs.v)
    }
}

impl ops::Div<f64> for Quat {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(rhs != 0.0);
        Self::from_sv(self.s / rhs, self.v / rhs)
    }
}

// impl ops::Div<Quat> for f64 {
//     type Output = Quat;

//     fn div(self, rhs: Quat) -> Quat {
//         debug_assert!(rhs != Quat::zero());
//         Quat::from_sv(self / rhs.s, self / rhs.v)
//     }
// }

impl ops::Rem<f64> for Quat {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Self::from_sv(self.s % rhs, self.v % rhs)
    }
}

impl ops::AddAssign for Quat {
    fn add_assign(&mut self, rhs: Self) {
        self.s += rhs.s;
        self.v += rhs.v;
    }
}

impl ops::SubAssign for Quat {
    fn sub_assign(&mut self, rhs: Self) {
        self.s -= rhs.s;
        self.v -= rhs.v;
    }
}

impl ops::MulAssign<f64> for Quat {
    fn mul_assign(&mut self, rhs: f64) {
        self.s *= rhs;
        self.v *= rhs;
    }
}

impl ops::DivAssign<f64> for Quat {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0);
        self.s /= rhs;
        self.v /= rhs;
    }
}

impl ops::RemAssign<f64> for Quat {
    fn rem_assign(&mut self, rhs: f64) {
        self.s %= rhs;
        self.v %= rhs;
    }
}

impl iter::Sum<Quat> for Quat {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), ops::Add::add)
    }
}

impl iter::Product<Quat> for Quat {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), ops::Mul::mul)
    }
}

impl From<Euler> for Quat {
    fn from(value: Euler) -> Self {
        // Euclidean Space has an Euler to quat equation, but it is for a different order (YXZ):
        // http://www.euclideanspace.com/maths/geometry/rotations/conversions/eulerToQuaternion/index.htm
        // Page A-2 here has the formula for XYZ:
        // http://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf

        let (s_x, c_x) = (value.x * 0.5).sin_cos();
        let (s_y, c_y) = (value.y * 0.5).sin_cos();
        let (s_z, c_z) = (value.z * 0.5).sin_cos();

        Self::new(
            -s_x * s_y * s_z + c_x * c_y * c_z,
            s_x * c_y * c_z + s_y * s_z * c_x,
            -s_x * s_z * c_y + s_y * c_x * c_z,
            s_x * s_y * c_z + s_z * c_x * c_y,
        )
    }
}
