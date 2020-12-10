use std::{fmt, iter, ops};

use super::Rad;

/// An angle, in degrees.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Deg(pub f64);

impl Deg {
    /// The zero angle.
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// A full frequency (1 / cycle).
    pub const fn frequency() -> Self {
        Self(0.0027777777777777778837886568652493224363_64)
    }

    /// A double frequency (2 / cycle).
    pub const fn frequency_x2() -> Self {
        Self(0.0055555555555555557675773137304986448726_64)
    }

    /// A full cycle.
    pub const fn cycle() -> Self {
        Self(360.0)
    }

    /// A half cycle.
    pub const fn half_cycle() -> Self {
        Self(180.0)
    }

    /// A quater cycle.
    pub const fn quat_cycle() -> Self {
        Self(90.0)
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
        Rad::from(self).sin()
    }

    /// Compute the cosine of the angle, returning a unitless ratio.
    pub fn cos(self) -> f64 {
        Rad::from(self).cos()
    }

    /// Compute the tangent of the angle, returning a unitless ratio.
    pub fn tan(self) -> f64 {
        Rad::from(self).tan()
    }

    /// Compute the sine and cosine of the angle, returning the result as a pair.
    pub fn sin_cos(self) -> (f64, f64) {
        Rad::from(self).sin_cos()
    }

    /// Compute the cosecant of the angle.
    pub fn csc(self) -> f64 {
        Rad::from(self).csc()
    }

    /// Compute the secant of the angle.
    pub fn sec(self) -> f64 {
        Rad::from(self).sec()
    }

    /// Compute the cotangent of the angle.
    pub fn cot(self) -> f64 {
        Rad::from(self).cot()
    }

    /// Compute the arcsine of the ratio, returning the resulting angle.
    pub fn asin(a: f64) -> Self {
        Rad::asin(a).into()
    }

    /// Compute the arccosine of the ratio, returning the resulting angle.
    pub fn acos(a: f64) -> Self {
        Rad::acos(a).into()
    }

    /// Compute the arctangent of the ratio, returning the resulting angle.
    pub fn atan(a: f64) -> Self {
        Rad::atan(a).into()
    }

    /// Computes the arctangent of the a / b, returning the resulting angle.
    pub fn atan2(a: f64, b: f64) -> Self {
        Rad::atan2(a, b).into()
    }
}

impl ops::Neg for Deg {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl ops::Add for Deg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl ops::Sub for Deg {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::Mul for Deg {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl ops::Mul<f64> for Deg {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Div for Deg {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        debug_assert!(rhs.0 != 0.0);
        Self(self.0 / rhs.0)
    }
}

impl ops::Div<f64> for Deg {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(rhs != 0.0);
        Self(self.0 / rhs)
    }
}

impl ops::Rem for Deg {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl ops::AddAssign for Deg {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::SubAssign for Deg {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl ops::MulAssign for Deg {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl ops::MulAssign<f64> for Deg {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl ops::DivAssign for Deg {
    fn div_assign(&mut self, rhs: Self) {
        debug_assert!(rhs.0 != 0.0);
        self.0 /= rhs.0;
    }
}

impl ops::DivAssign<f64> for Deg {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0);
        self.0 /= rhs;
    }
}

impl ops::RemAssign for Deg {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl ops::Rem<f64> for Deg {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Self(self.0 % rhs)
    }
}

impl ops::RemAssign<f64> for Deg {
    fn rem_assign(&mut self, rhs: f64) {
        self.0 %= rhs;
    }
}

impl iter::Sum for Deg {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), ops::Add::add)
    }
}

impl fmt::Display for Deg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f64> for Deg {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Rad> for Deg {
    fn from(value: Rad) -> Self {
        Self((value * Rad::frequency()).0 * Self::cycle().0)
    }
}
