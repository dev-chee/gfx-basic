use std::{fmt, ops};

use crate::vector::{Vec3, Vec4};

use super::Pt2;

/// A point in 3-dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pt3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pt3 {
    /// The point at the origin of the space.
    pub const fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Construct a new point, using the provided values.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Peform the approx equality comparison.
    pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
        (self.x - rhs.x).abs() < epsilon.x
            && (self.y - rhs.y).abs() < epsilon.y
            && (self.z - rhs.z).abs() < epsilon.z
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

    pub fn from_homogeneous(value: Vec4) -> Self {
        let p = 1.0 / value.w;
        Self {
            x: value.x * p,
            y: value.y * p,
            z: value.z * p,
        }
    }

    pub fn to_homogeneous(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 0.0)
    }
}

impl ops::Neg for Pt3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<Vec3> for Pt3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Pt3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Pt3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Pt3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Pt3> for f64 {
    type Output = Pt3;

    fn mul(self, rhs: Pt3) -> Pt3 {
        Pt3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Pt3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Pt3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(rhs != 0.0);
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Pt3 {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0);
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Rem<f64> for Pt3 {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}

impl ops::RemAssign<f64> for Pt3 {
    fn rem_assign(&mut self, rhs: f64) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
    }
}

impl ops::Index<usize> for Pt3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Pt3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of range"),
        }
    }
}

impl From<f64> for Pt3 {
    fn from(value: f64) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl From<[f64; 3]> for Pt3 {
    fn from(value: [f64; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<Vec3> for Pt3 {
    fn from(value: Vec3) -> Self {
        Self::origin() + value
    }
}

impl Into<[f64; 3]> for Pt3 {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl Into<(f64, f64, f64)> for Pt3 {
    fn into(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Pt3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<Pt2> for Pt3 {
    fn from(value: Pt2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: 0.0,
        }
    }
}

impl fmt::Display for Pt3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
