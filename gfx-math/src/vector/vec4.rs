use std::{fmt, iter, ops};

use crate::angle::Rad;

use super::{Vec2, Vec3};

/// A 4-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub const fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub const fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub const fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }

    pub const fn unit_w() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Construct a new vector, using the provided values.
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Computes the absolute value.
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    /// Peform the approx equality comparison.
    pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
        (self.x - rhs.x).abs() < epsilon.x
            && (self.y - rhs.y).abs() < epsilon.y
            && (self.z - rhs.z).abs() < epsilon.z
            && (self.w - rhs.w).abs() < epsilon.w
    }

    /// Vector dot (or inner) product.
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
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

impl ops::Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl ops::Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl ops::Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl ops::MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl ops::MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl ops::Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        debug_assert!(rhs != Self::zero());
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl ops::Div<f64> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(rhs != 0.0);
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        debug_assert!(rhs != Self::zero());
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl ops::DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0);
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl ops::Rem for Vec4 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
            w: self.w % rhs.w,
        }
    }
}

impl ops::Rem<f64> for Vec4 {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
            w: self.w % rhs,
        }
    }
}

impl ops::RemAssign for Vec4 {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
        self.w %= rhs.w;
    }
}

impl ops::RemAssign<f64> for Vec4 {
    fn rem_assign(&mut self, rhs: f64) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
        self.w %= rhs;
    }
}

impl ops::Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("out of range"),
        }
    }
}

impl iter::Sum for Vec4 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), ops::Add::add)
    }
}

impl From<f64> for Vec4 {
    fn from(value: f64) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }
}

impl From<[f64; 4]> for Vec4 {
    fn from(value: [f64; 4]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        }
    }
}

impl Into<[f64; 4]> for Vec4 {
    fn into(self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl From<(f64, f64, f64, f64)> for Vec4 {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
            w: value.3,
        }
    }
}

impl From<Vec2> for Vec4 {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: 0.0,
        }
    }
}

impl Into<(f64, f64, f64, f64)> for Vec4 {
    fn into(self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, self.w)
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
