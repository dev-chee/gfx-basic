use std::{fmt, iter, ops};

use crate::{angle::Rad, point::Pt2};

use super::{Vec3, Vec4};

/// A 2-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub const fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    pub const fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    /// Construct a new vector, using the provided values.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Computes the absolute value.
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Peform the approx equality comparison.
    pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
        (self.x - rhs.x).abs() < epsilon.x && (self.y - rhs.y).abs() < epsilon.y
    }

    /// Vector dot (or inner) product.
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
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

    /// The perpendicular dot product of the vector and `rhs`.
    pub fn perp_dot(self, rhs: Self) -> f64 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        debug_assert!(rhs != Self::zero());
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(rhs != 0.0);
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        debug_assert!(rhs != Self::zero());
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0);
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Rem for Vec2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl ops::Rem<f64> for Vec2 {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl ops::RemAssign for Vec2 {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl ops::RemAssign<f64> for Vec2 {
    fn rem_assign(&mut self, rhs: f64) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

impl ops::Index<usize> for Vec2 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("out of range"),
        }
    }
}

impl iter::Sum for Vec2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), ops::Add::add)
    }
}

impl From<f64> for Vec2 {
    fn from(value: f64) -> Self {
        Self { x: value, y: value }
    }
}

impl From<[f64; 2]> for Vec2 {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl Into<[f64; 2]> for Vec2 {
    fn into(self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl ops::Add<Pt2> for Vec2 {
    type Output = Pt2;

    fn add(self, rhs: Pt2) -> Pt2 {
        Pt2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl From<Pt2> for Vec2 {
    fn from(value: Pt2) -> Self {
        value - Pt2::origin()
    }
}

impl<'a> From<&'a Pt2> for Vec2 {
    fn from(value: &'a Pt2) -> Self {
        *value - Pt2::origin()
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Vec3> for Vec2 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vec4> for Vec2 {
    fn from(value: Vec4) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Into<(f64, f64)> for Vec2 {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
