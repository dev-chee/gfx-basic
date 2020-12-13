macro_rules! define_angle {
    ($Angle:ident) => {
        #[derive(Debug, Default, Copy, Clone)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $Angle<N>(pub(crate) N);

        impl<N> $Angle<N> {
            #[inline]
            pub const fn new(angle: N) -> Self {
                Self(angle)
            }

            #[inline]
            pub const fn inner(&self) -> &N {
                &self.0
            }

            #[inline]
            pub fn inner_mut(&mut self) -> &mut N {
                &mut self.0
            }
        }

        impl<N: std::fmt::Display> std::fmt::Display for $Angle<N> {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<N: ApproxEq> PartialEq for $Angle<N> {
            #[inline]
            fn eq(&self, rhs: &Self) -> bool {
                self.0.approx_eq(&rhs.0)
            }
        }

        impl<N: PartialOrd + ApproxEq> PartialOrd for $Angle<N> {
            #[inline]
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&rhs.0)
            }
        }

        impl<N: std::ops::Neg<Output = N>> std::ops::Neg for $Angle<N> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self::new(-self.0)
            }
        }

        impl<N: std::ops::Add<Output = N>> std::ops::Add for $Angle<N> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                $Angle::new(self.0 + rhs.0)
            }
        }

        impl<N: std::ops::AddAssign> std::ops::AddAssign for $Angle<N> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl<N: std::ops::Sub<Output = N>> std::ops::Sub for $Angle<N> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                $Angle::new(self.0 - rhs.0)
            }
        }

        impl<N: std::ops::SubAssign> std::ops::SubAssign for $Angle<N> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl<N: std::ops::Mul<Output = N>> std::ops::Mul<N> for $Angle<N> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: N) -> Self {
                Self::new(self.0 * rhs)
            }
        }

        impl<N: std::ops::MulAssign> std::ops::MulAssign<N> for $Angle<N> {
            #[inline]
            fn mul_assign(&mut self, rhs: N) {
                self.0 *= rhs;
            }
        }

        impl<N: std::ops::Div<Output = N>> std::ops::Div for $Angle<N> {
            type Output = N;

            #[inline]
            fn div(self, rhs: Self) -> N {
                self.0 / rhs.0
            }
        }

        impl<N: std::ops::Div<Output = N>> std::ops::Div<N> for $Angle<N> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: N) -> Self {
                Self::new(self.0 / rhs)
            }
        }

        impl<N: std::ops::DivAssign> std::ops::DivAssign<N> for $Angle<N> {
            #[inline]
            fn div_assign(&mut self, rhs: N) {
                self.0 /= rhs;
            }
        }

        impl<N: std::ops::Rem<Output = N>> std::ops::Rem<N> for $Angle<N> {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: N) -> Self {
                $Angle::new(self.0 % rhs)
            }
        }

        impl<N: std::ops::RemAssign> std::ops::RemAssign<N> for $Angle<N> {
            #[inline]
            fn rem_assign(&mut self, rhs: N) {
                self.0 %= rhs
            }
        }
    };
}

macro_rules! impl_angle {
    ($Angle:ident, $Num:ty, $cycle:expr, $($Src:ident),*) => {
        impl $Angle<$Num> {
            #[inline]
            pub const fn zero() -> Self {
                Self::new(0.0)
            }

            #[inline]
            pub fn quat() -> Self {
                Self::new($cycle * 0.25)
            }

            #[inline]
            pub fn half() -> Self {
                Self::new($cycle * 0.5)
            }

            #[inline]
            pub fn full() -> Self {
                Self::new($cycle)
            }

            #[inline]
            pub const fn cycle() -> $Num {
                $cycle
            }

            #[inline]
            pub fn freq() -> $Num {
                1.0 / $cycle
            }

            #[inline]
            pub fn norm(self) -> Self {
                let mut rem = self % Self::cycle();
                if rem < Self::zero() {
                    rem += Self::full()
                }
                rem
            }

            #[inline]
            pub fn to_norm(&mut self) -> Self {
                *self %= Self::cycle();
                if *self < Self::zero() {
                    *self += Self::full()
                }
                *self
            }

            #[inline]
            pub fn abs(&self) -> Self {
                Self::new(self.0.abs())
            }
        }

        $(
            impl From<$Src<$Num>> for $Angle<$Num> {
                #[inline]
                fn from(value: $Src<$Num>) -> Self {
                    Self::full() * (value.0 * $Src::freq())
                }
            }
        )*
    };
}

macro_rules! impl_rad {
    ($Num:ty) => {
        impl Rad<$Num> {
            #[inline]
            pub fn sin(&self) -> $Num {
                self.0.sin()
            }

            #[inline]
            pub fn cos(&self) -> $Num {
                self.0.cos()
            }

            #[inline]
            pub fn tan(&self) -> $Num {
                self.0.tan()
            }

            #[inline]
            pub fn sin_cos(self) -> ($Num, $Num) {
                self.0.sin_cos()
            }

            #[inline]
            pub fn asin(a: $Num) -> Rad<$Num> {
                Self::new(a.asin())
            }

            #[inline]
            pub fn acos(a: $Num) -> Rad<$Num> {
                Self::new(a.acos())
            }

            #[inline]
            pub fn atan(a: $Num) -> Rad<$Num> {
                Self::new(a.atan())
            }

            #[inline]
            pub fn atan2(a: $Num, b: $Num) -> Rad<$Num> {
                Self::new(a.atan2(b))
            }
        }
    };
}

macro_rules! impl_deg {
    ($Num:ty) => {
        impl Deg<$Num> {
            #[inline]
            pub fn sin(&self) -> $Num {
                Rad::<_>::from(*self).sin()
            }

            #[inline]
            pub fn cos(&self) -> $Num {
                Rad::<_>::from(*self).cos()
            }

            #[inline]
            pub fn tan(&self) -> $Num {
                Rad::<_>::from(*self).tan()
            }

            #[inline]
            pub fn sin_cos(&self) -> ($Num, $Num) {
                Rad::<_>::from(*self).sin_cos()
            }

            #[inline]
            pub fn asin(a: $Num) -> Deg<$Num> {
                Deg::from(Rad::asin(a))
            }

            #[inline]
            pub fn acos(a: $Num) -> Deg<$Num> {
                Deg::from(Rad::acos(a))
            }

            #[inline]
            pub fn atan(a: $Num) -> Deg<$Num> {
                Deg::from(Rad::atan(a))
            }

            #[inline]
            pub fn atan2(a: $Num, b: $Num) -> Deg<$Num> {
                Deg::from(Rad::atan2(a, b))
            }
        }
    };
}
