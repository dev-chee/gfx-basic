macro_rules! define_vec {
    ($Vec:ident, $N:ty, $n:expr, { $($f:ident : $i:expr),+ }) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        pub struct $Vec([$N; $n]);

        impl $Vec {
            #[inline]
            pub const fn zero() -> Self {
                Self([0.0; $n])
            }

            #[inline]
            pub const fn new($($f:$N),+) -> Self {
                Self([$($f),+])
            }

            $(
                #[inline]
                pub const fn $f(self) -> f64 {
                    self.0[$i]
                }

                define_vec!(@unit, $f);
            )+




            // #[inline]
            // pub const fn unit_y() -> Self {
            //     Self::new(0.0, 1.0)
            // }
        }

        impl fmt::Display for Vec2 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.x(), self.y())
            }
        }

        impl From<f64> for Vec2 {
            fn from(value: f64) -> Self {
                Self::new(value, value)
            }
        }
    };
    (@unit, $name:ident) => {
        #[inline]
        pub const fn unit_$name() -> Self {
            Self::new(1.0, 0.0)
        }
    }
}
