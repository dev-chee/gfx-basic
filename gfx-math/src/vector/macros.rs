macro_rules! define_vec {
    ($Vec:ident, $Pt:ident, $N:ty, $eps:expr, $n:expr, { $($f:ident : $set_f:ident : $i:expr),+ }, $($unit:ident : $val:expr),+) => {
        #[derive(Debug, Default, Copy, Clone)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $Vec(pub(crate) [$N; $n]);

        impl $Vec {
            #[inline]
            pub const fn zero() -> Self {
                Self([0.0; $n])
            }

            #[inline]
            pub const fn epsilon() -> Self {
                Self([$eps; $n])
            }

            #[inline]
            pub const fn new($($f:$N),+) -> Self {
                Self([$($f),+])
            }

            $(
                #[inline]
                pub const fn $f(&self) -> $N {
                    self.0[$i]
                }

                #[inline]
                pub fn $set_f(&mut self, value: $N) -> &mut Self {
                    self.0[$i] = value;
                    self
                }
            )+

            $(
                #[inline]
                pub const fn $unit() -> Self {
                    Self($val)
                }
            )+

            #[inline]
            pub fn dot(&self, rhs: &Self) -> $N {
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .fold(0.0, |sum, i| sum + i.0 * i.1)
            }

            #[inline]
            pub fn norm(&self) -> Self {
                (*self) / self.len()
            }

            #[inline]
            pub fn to_norm(&mut self) -> &mut Self {
                (*self) /= self.len();
                self
            }

            #[inline]
            pub fn len(&self) -> $N {
                self.len_sq2().sqrt()
            }

            #[inline]
            pub fn len_sq2(&self) -> $N {
                self.dot(self)
            }
        }

        impl PartialEq for $Vec {
            #[inline]
            fn eq(&self, rhs: &Self) -> bool {
                for i in self.0.iter().zip(&rhs.0) {
                    if (i.0 - i.1).abs() >= $eps {
                        return false;
                    }
                }

                true
            }
        }

        impl ops::Neg for $Vec {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                let mut v = self;
                v.0.iter_mut().for_each(|i| *i = -*i);
                v
            }
        }

        impl ops::Add for $Vec {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                let mut v = self;
                v.0.iter_mut().zip(rhs.0.iter()).for_each(|i| *i.0 += i.1);
                v
            }
        }

        impl ops::Add<$Pt> for $Vec {
            type Output = $Pt;

            #[inline]
            fn add(self, rhs: $Pt) -> $Pt {
                let mut v = rhs;
                v.0.iter_mut().zip(self.0.iter()).for_each(|i| *i.0 += i.1);
                v
            }
        }

        impl ops::AddAssign for $Vec {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0
                    .iter_mut()
                    .zip(rhs.0.iter())
                    .for_each(|i| *i.0 += i.1);
            }
        }

        impl ops::Sub for $Vec {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                let mut v = self;
                v.0.iter_mut().zip(rhs.0.iter()).for_each(|i| *i.0 -= i.1);
                v
            }
        }

        impl ops::SubAssign for $Vec {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0
                    .iter_mut()
                    .zip(rhs.0.iter())
                    .for_each(|i| *i.0 -= i.1);
            }
        }

        impl ops::Mul<$N> for $Vec {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $N) -> Self {
                let mut v = self;
                v.0.iter_mut().for_each(|i| *i *= rhs);
                v
            }
        }

        impl ops::Mul<$Vec> for $N {
            type Output = $Vec;

            #[inline]
            fn mul(self, rhs: $Vec) -> $Vec {
                let mut v = rhs;
                v.0.iter_mut().for_each(|i| *i *= self);
                v
            }
        }

        impl ops::MulAssign<$N> for $Vec {
            #[inline]
            fn mul_assign(&mut self, rhs: $N) {
                self.0.iter_mut().for_each(|i| *i *= rhs);
            }
        }

        impl ops::Div<$N> for $Vec {
            type Output = Self;

            #[inline]
            fn div(self, rhs: $N) -> Self {
                debug_assert!(rhs != 0.0);
                let mut v = self;
                v.0.iter_mut().for_each(|i| *i /= rhs);
                v
            }
        }

        impl ops::Div<$Vec> for $N {
            type Output = $Vec;

            #[inline]
            fn div(self, rhs: $Vec) -> $Vec {
                debug_assert!(self != 0.0);
                let mut v = rhs;
                v.0.iter_mut().for_each(|i| *i /= self);
                v
            }
        }

        impl ops::DivAssign<$N> for $Vec {
            #[inline]
            fn div_assign(&mut self, rhs: $N) {
                debug_assert!(rhs != 0.0);
                self.0.iter_mut().for_each(|i| *i /= rhs);
            }
        }

        impl ops::Rem<$N> for $Vec {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: $N) -> Self {
                let mut v = self;
                v.0.iter_mut().for_each(|i| *i %= rhs);
                v
            }
        }

        impl ops::Rem<$Vec> for $N {
            type Output = $Vec;

            #[inline]
            fn rem(self, rhs: $Vec) -> $Vec {
                let mut v = rhs;
                v.0.iter_mut().for_each(|i| *i %= self);
                v
            }
        }

        impl ops::RemAssign<$N> for $Vec {
            #[inline]
            fn rem_assign(&mut self, rhs: $N) {
                self.0.iter_mut().for_each(|i| *i %= rhs);
            }
        }

        impl ops::Index<usize> for $Vec {
            type Output = $N;

            #[inline]
            fn index(&self, index: usize) -> &$N {
                if index >= $n {
                    panic!("Out of Range");
                }
                &self.0[index]
            }
        }

        impl ops::IndexMut<usize> for $Vec {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut $N {
                if index >= $n {
                    panic!("Out of Range");
                }
                &mut self.0[index]
            }
        }

        impl From<$N> for $Vec {
            #[inline]
            fn from(value: $N) -> Self {
                Self([value;$n])
            }
        }

        impl From<[$N; $n]> for $Vec {
            #[inline]
            fn from(value: [$N; $n]) -> Self {
                Self(value)
            }
        }

        impl Into<[$N; $n]> for $Vec {
            #[inline]
            fn into(self) -> [$N; $n] {
                self.0
            }
        }

        impl From<$Pt> for $Vec {
            fn from(value: $Pt) -> Self {
                value - $Pt::origin()
            }
        }
    };
}
