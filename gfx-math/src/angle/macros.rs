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

            /// Peform the approx equality comparison.
            pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
                (self.0 - rhs.0).abs() < epsilon.0
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

        impl From<$T> for $Angle<$T> {
            fn from(value: $T) -> Self {
                Self(value)
            }
        }
    };
}
