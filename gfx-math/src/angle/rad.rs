use std::{f64, fmt, iter, ops};

use super::Deg;

/// An angle, in radians.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rad(pub f64);

impl Rad {
    /// The zero angle.
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// A full frequency (1 / cycle).
    pub const fn frequency() -> Self {
        Self(0.1591549430918953456082221009637578390539_f64)
    }

    /// A double frequency (2 / cycle).
    pub const fn frequency_x2() -> Self {
        Self(f64::consts::FRAC_1_PI)
    }

    /// A full cycle.
    pub const fn cycle() -> Self {
        Self(f64::consts::TAU)
    }

    /// A half cycle.
    pub const fn half_cycle() -> Self {
        Self(f64::consts::PI)
    }

    /// A quater cycle.
    pub const fn quat_cycle() -> Self {
        Self(1.57079632679489661923132169163975144_f64)
    }

    /// Return normalized to the range `[0, FULL_TURN)`.
    pub fn normalize(self) -> Self {
        let rem = self % Self::cycle();
        if rem < Self::zero() {
            rem + Self::cycle()
        } else {
            rem
        }
    }

    /// Return the angle, normalized to the range `[-HALF_TURN, HALF_TURN)`.
    pub fn normalize_signed(self) -> Self {
        self.normalize() - Self::half_cycle()
    }

    /// Returns the linear interior of the two angles.
    pub fn lerp(self, rhs: Self, t: f64) -> Self {
        self + (rhs - self) * t
    }

    /// Peform the approx equality comparison.
    pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
        (self.0 - rhs.0).abs() < epsilon.0
    }

    /// Compute the sine of the angle, returning a unitless ratio.
    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    /// Compute the cosine of the angle, returning a unitless ratio.
    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    /// Compute the tangent of the angle, returning a unitless ratio.
    pub fn tan(self) -> f64 {
        self.0.tan()
    }

    /// Compute the sine and cosine of the angle, returning the result as a pair.
    pub fn sin_cos(self) -> (f64, f64) {
        self.0.sin_cos()
    }

    /// Compute the cosecant of the angle.
    pub fn csc(self) -> f64 {
        self.sin().recip()
    }

    /// Compute the secant of the angle.
    pub fn sec(self) -> f64 {
        self.cos().recip()
    }

    /// Compute the cotangent of the angle.
    pub fn cot(self) -> f64 {
        self.tan().recip()
    }

    /// Compute the arcsine of the ratio, returning the resulting angle.
    pub fn asin(a: f64) -> Self {
        Self(a.asin())
    }

    /// Compute the arccosine of the ratio, returning the resulting angle.
    pub fn acos(a: f64) -> Self {
        Self(a.acos())
    }

    /// Compute the arctangent of the ratio, returning the resulting angle.
    pub fn atan(a: f64) -> Self {
        Self(a.atan())
    }

    /// Computes the arctangent of the a / b, returning the resulting angle.
    pub fn atan2(a: f64, b: f64) -> Self {
        Self(a.atan2(b))
    }
}

impl ops::Neg for Rad {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl ops::Add for Rad {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl ops::Sub for Rad {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::Mul for Rad {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl ops::Mul<f64> for Rad {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Div for Rad {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        debug_assert!(rhs.0 != 0.0);
        Self(self.0 / rhs.0)
    }
}

impl ops::Div<f64> for Rad {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(rhs != 0.0);
        Self(self.0 / rhs)
    }
}

impl ops::Rem for Rad {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl ops::AddAssign for Rad {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::SubAssign for Rad {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ops::MulAssign for Rad {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl ops::MulAssign<f64> for Rad {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl ops::DivAssign for Rad {
    fn div_assign(&mut self, rhs: Self) {
        debug_assert!(rhs.0 != 0.0);
        self.0 /= rhs.0;
    }
}

impl ops::DivAssign<f64> for Rad {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0);
        self.0 /= rhs;
    }
}

impl ops::RemAssign for Rad {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl ops::Rem<f64> for Rad {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Self(self.0 % rhs)
    }
}

impl ops::RemAssign<f64> for Rad {
    fn rem_assign(&mut self, rhs: f64) {
        self.0 %= rhs;
    }
}

impl iter::Sum for Rad {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), ops::Add::add)
    }
}

impl fmt::Display for Rad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f64> for Rad {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Deg> for Rad {
    fn from(value: Deg) -> Self {
        Self((value * Deg::frequency()).0 * Self::cycle().0)
    }
}
