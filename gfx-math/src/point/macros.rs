macro_rules! define_pt {
    ($Pt:ident, $Vec:ident, $N:ty, $eps:expr, $n:expr, { $($f:ident : $set_f:ident : $i:expr),+ }) => {
        #[derive(Debug, Default, Copy, Clone)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $Pt(pub(crate) [$N; $n]);

        impl $Pt {
            #[inline]
            pub const fn epsilon() -> Self {
                Self([$eps; $n])
            }

            #[inline]
            pub const fn origin() -> Self {
                Self([0.0; $n])
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
        }

        impl PartialEq for $Pt {
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

        impl ops::Neg for $Pt {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                let mut v = self;
                v.0.iter_mut().for_each(|i| *i = -*i);
                v
            }
        }

        impl ops::Add<$Vec> for $Pt {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $Vec) -> Self {
                let mut v = self.clone();
                v.0.iter_mut().zip(rhs.0.iter()).for_each(|i| *i.0 += i.1);
                v
            }
        }

        impl ops::AddAssign<$Vec> for $Pt {
            #[inline]
            fn add_assign(&mut self, rhs: $Vec) {
                self.0
                    .iter_mut()
                    .zip(rhs.0.iter())
                    .for_each(|i| *i.0 += i.1);
            }
        }

        impl ops::Sub for $Pt {
            type Output = $Vec;

            #[inline]
            fn sub(self, rhs: Self) -> $Vec {
                let mut v = $Vec::from(self);
                v.0.iter_mut().zip(rhs.0.iter()).for_each(|i| *i.0 -= i.1);
                v
            }
        }

        impl ops::Sub<$Vec> for $Pt {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $Vec) -> Self {
                let mut v = self;
                v.0.iter_mut().zip(rhs.0.iter()).for_each(|i| *i.0 -= i.1);
                v
            }
        }

        impl ops::SubAssign<$Vec> for $Pt {
            #[inline]
            fn sub_assign(&mut self, rhs: $Vec) {
                self.0
                    .iter_mut()
                    .zip(rhs.0.iter())
                    .for_each(|i| *i.0 -= i.1);
            }
        }

        impl ops::Index<usize> for $Pt {
            type Output = $N;

            #[inline]
            fn index(&self, index: usize) -> &$N {
                if index >= $n {
                    panic!("Out of Range");
                }
                &self.0[index]
            }
        }

        impl ops::IndexMut<usize> for $Pt {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut $N {
                if index >= $n {
                    panic!("Out of Range");
                }
                &mut self.0[index]
            }
        }

        impl From<$Vec> for $Pt {
            #[inline]
            fn from(value: $Vec) -> Self {
                Self::origin() + value
            }
        }

        impl From<$N> for $Pt {
            #[inline]
            fn from(value: $N) -> Self {
                Self([value;$n])
            }
        }

        impl From<[$N; $n]> for $Pt {
            #[inline]
            fn from(value: [$N; $n]) -> Self {
                Self(value)
            }
        }

        impl Into<[$N; $n]> for $Pt {
            #[inline]
            fn into(self) -> [$N; $n] {
                self.0
            }
        }
    };
}
