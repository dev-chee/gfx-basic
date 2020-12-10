use std::{fmt, ops};

use crate::vector::{Vec2, Vec3};

use super::Pt3;

/// A point in 2-dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pt2 {
    pub x: f64,
    pub y: f64,
}

impl Pt2 {
    /// The point at the origin of the space.
    pub const fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Construct a new point, using the provided values.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Peform the approx equality comparison.
    pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
        (self.x - rhs.x).abs() < epsilon.x && (self.y - rhs.y).abs() < epsilon.y
    }

    /// Returns the squared distance.
    pub fn distance2(self, rhs: Self) -> f64 {
        (rhs - self).magnitude2()
    }

    /// The distance between two values.
    pub fn distance(self, rhs: Self) -> f64 {
        (rhs - self).magnitude()
    }

    /// Returns the result of linearly interpolating the vector
    /// towards `rhs` by the specified amount.
    pub fn lerp(self, rhs: Self, t: f64) -> Self {
        self + ((rhs - self) * t)
    }

    /// Returns the average position of all points in the slice.
    pub fn centroid(points: &[Self]) -> Self {
        let total_displacement = points.iter().fold(Self::origin(), |acc, p| acc + p.into());
        total_displacement / (points.len() as f64)
    }

    pub fn from_homogeneous(value: Vec3) -> Self {
        let p = 1.0 / value.z;
        Self {
            x: value.x * p,
            y: value.y * p,
        }
    }

    pub fn to_homogeneous(self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl ops::Neg for Pt2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add<Vec2> for Pt2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vec2> for Pt2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Pt2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<f64> for Pt2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Pt2> for f64 {
    type Output = Pt2;

    fn mul(self, rhs: Pt2) -> Pt2 {
        Pt2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::MulAssign<f64> for Pt2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<f64> for Pt2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(rhs != 0.0);
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Pt2 {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0);
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Rem<f64> for Pt2 {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl ops::RemAssign<f64> for Pt2 {
    fn rem_assign(&mut self, rhs: f64) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

impl ops::Index<usize> for Pt2 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Pt2 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("out of range"),
        }
    }
}

impl From<f64> for Pt2 {
    fn from(value: f64) -> Self {
        Self { x: value, y: value }
    }
}

impl From<[f64; 2]> for Pt2 {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<Vec2> for Pt2 {
    fn from(value: Vec2) -> Self {
        Self::origin() + value
    }
}

impl Into<[f64; 2]> for Pt2 {
    fn into(self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl Into<(f64, f64)> for Pt2 {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl From<(f64, f64)> for Pt2 {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Pt3> for Pt2 {
    fn from(value: Pt3) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl fmt::Display for Pt2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
