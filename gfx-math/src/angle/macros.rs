macro_rules! impl_angle {
    ($Angle:ident < $T:ty > { $zero:expr, $unit:expr, $full:expr, $half:expr, $quart:expr}) => {
        impl $Angle<$T> {
            pub const ZERO_TURN: Self = Self($zero);
            pub const UNIT_TURN: Self = Self($unit);
            pub const FULL_TURN: Self = Self($full);
            pub const HALF_TURN: Self = Self($half);
            pub const QUART_TURN: Self = Self($quart);

            /// Return the angle, normalized to the range `[0, FULL_TURN)`.
            pub fn normalize(self) -> Self {
                let rem = self % Self::FULL_TURN;
                if rem < Self::ZERO_TURN {
                    rem + Self::FULL_TURN
                } else {
                    rem
                }
            }

            /// Return the angle, normalized to the range `[-HALF_TURN, HALF_TURN)`.
            pub fn normalize_signed(self) -> Self {
                self.normalize() - Self::HALF_TURN
            }

            /// Returns the linear interior of the two angles.
            pub fn lerp(self, rhs: Self, t: $T) -> Self {
                self + (rhs - self) * t
            }
        }

        impl ops::Neg for $Angle<$T> {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl ops::Add for $Angle<$T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl ops::Sub for $Angle<$T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl ops::Mul for $Angle<$T> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }
        }

        impl ops::Div for $Angle<$T> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                debug_assert!(rhs.0 != $zero);
                Self(self.0 / rhs.0)
            }
        }

        impl ops::Rem for $Angle<$T> {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self {
                Self(self.0 % rhs.0)
            }
        }

        impl ops::AddAssign for $Angle<$T> {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl ops::SubAssign for $Angle<$T> {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl ops::MulAssign for $Angle<$T> {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }

        impl ops::DivAssign for $Angle<$T> {
            fn div_assign(&mut self, rhs: Self) {
                debug_assert!(rhs.0 != $zero);
                self.0 /= rhs.0;
            }
        }

        impl ops::RemAssign for $Angle<$T> {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }

        impl ops::Mul<$T> for $Angle<$T> {
            type Output = Self;

            fn mul(self, rhs: $T) -> Self {
                Self(self.0 * rhs)
            }
        }

        impl ops::Div<$T> for $Angle<$T> {
            type Output = Self;

            fn div(self, rhs: $T) -> Self {
                debug_assert!(rhs != $zero);
                Self(self.0 / rhs)
            }
        }

        impl ops::Rem<$T> for $Angle<$T> {
            type Output = Self;

            fn rem(self, rhs: $T) -> Self {
                Self(self.0 % rhs)
            }
        }

        impl ops::MulAssign<$T> for $Angle<$T> {
            fn mul_assign(&mut self, rhs: $T) {
                self.0 *= rhs;
            }
        }

        impl ops::DivAssign<$T> for $Angle<$T> {
            fn div_assign(&mut self, rhs: $T) {
                debug_assert!(rhs != $zero);
                self.0 /= rhs;
            }
        }

        impl ops::RemAssign<$T> for $Angle<$T> {
            fn rem_assign(&mut self, rhs: $T) {
                self.0 %= rhs;
            }
        }

        impl iter::Sum for $Angle<$T> {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::ZERO_TURN, ops::Add::add)
            }
        }

        impl ops::Mul<$Angle<$T>> for $T {
            type Output = $Angle<$T>;

            fn mul(self, rhs: $Angle<$T>) -> $Angle<$T> {
                $Angle(self * rhs.0)
            }
        }
    };
}

macro_rules! impl_rad {
    ($T:ty) => {
        impl Rad<$T> {
            /// Compute the sine of the angle, returning a unitless ratio.
            pub fn sin(self) -> $T {
                self.0.sin()
            }

            /// Compute the cosine of the angle, returning a unitless ratio.
            pub fn cos(self) -> $T {
                self.0.cos()
            }

            /// Compute the tangent of the angle, returning a unitless ratio.
            pub fn tan(self) -> $T {
                self.0.tan()
            }

            /// Compute the sine and cosine of the angle, returning the result as a pair.
            pub fn sin_cos(self) -> ($T, $T) {
                self.0.sin_cos()
            }

            /// Compute the cosecant of the angle.
            pub fn csc(self) -> $T {
                self.sin().recip()
            }

            /// Compute the secant of the angle.
            pub fn sec(self) -> $T {
                self.cos().recip()
            }

            /// Compute the cotangent of the angle.
            pub fn cot(self) -> $T {
                self.tan().recip()
            }

            /// Compute the arcsine of the ratio, returning the resulting angle.
            pub fn asin(a: $T) -> Self {
                Self(a.asin())
            }

            /// Compute the arccosine of the ratio, returning the resulting angle.
            pub fn acos(a: $T) -> Self {
                Self(a.acos())
            }

            /// Compute the arctangent of the ratio, returning the resulting angle.
            pub fn atan(a: $T) -> Self {
                Self(a.atan())
            }

            /// Computes the arctangent of the a / b, returning the resulting angle.
            pub fn atan2(a: $T, b: $T) -> Self {
                Self(a.atan2(b))
            }
        }

        impl fmt::Debug for Rad<$T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}_rad", self.0)
            }
        }

        impl From<Deg<$T>> for Rad<$T> {
            fn from(deg: Deg<$T>) -> Self {
                Self(deg.0 * Self::HALF_TURN.0 / Deg::<$T>::HALF_TURN.0)
            }
        }
    };
}

macro_rules! impl_deg {
    ($T:ty) => {
        impl Deg<$T> {
            /// Compute the sine of the angle, returning a unitless ratio.
            pub fn sin(self) -> $T {
                Rad::from(self).sin()
            }

            /// Compute the cosine of the angle, returning a unitless ratio.
            pub fn cos(self) -> $T {
                Rad::from(self).cos()
            }

            /// Compute the tangent of the angle, returning a unitless ratio.
            pub fn tan(self) -> $T {
                Rad::from(self).tan()
            }

            /// Compute the sine and cosine of the angle, returning the result as a pair.
            pub fn sin_cos(self) -> ($T, $T) {
                Rad::from(self).sin_cos()
            }

            /// Compute the cosecant of the angle.
            pub fn csc(self) -> $T {
                Rad::from(self).csc()
            }

            /// Compute the secant of the angle.
            pub fn sec(self) -> $T {
                Rad::from(self).sec()
            }

            /// Compute the cotangent of the angle.
            pub fn cot(self) -> $T {
                Rad::from(self).cot()
            }

            /// Compute the arcsine of the ratio, returning the resulting angle.
            pub fn asin(a: $T) -> Self {
                Rad::<$T>::asin(a).into()
            }

            /// Compute the arccosine of the ratio, returning the resulting angle.
            pub fn acos(a: $T) -> Self {
                Rad::<$T>::acos(a).into()
            }

            /// Compute the arctangent of the ratio, returning the resulting angle.
            pub fn atan(a: $T) -> Self {
                Rad::<$T>::atan(a).into()
            }

            /// Computes the arctangent of the a / b, returning the resulting angle.
            pub fn atan2(a: $T, b: $T) -> Self {
                Rad::<$T>::atan2(a, b).into()
            }
        }

        impl fmt::Debug for Deg<$T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}_deg", self.0)
            }
        }

        impl From<Rad<$T>> for Deg<$T> {
            fn from(rad: Rad<$T>) -> Self {
                Self(rad.0 * Self::HALF_TURN.0 / Rad::<$T>::HALF_TURN.0)
            }
        }
    };
}
