macro_rules! impl_point {
    ($Vector:ident, $Point:ident < $T:ty >, $n:expr, { $($field:ident : $index:expr),+ }, $zero:expr, $one:expr) => {
        impl $Point<$T> {
            /// The point at the origin of the space.
            pub const ORIGIN:Self = Self { $($field: $zero),+};

            /// Construct a new point, using the provided values.
            pub const fn new($($field: $T),+) -> Self {
                Self { $($field),+ }
            }

            /// Peform the approx equality comparison.
            pub fn approx_eq(self, rhs: Self, epsilon: Self) -> bool {
                $((self.$field - rhs.$field).abs() < epsilon.$field)&&+
            }

            /// Perform the given operation on each field in the point, returning a new point
            /// constructed from the operations.
            pub fn map<F>(self, mut f: F) -> Self
                where F: FnMut($T) -> $T
            {
                Self::new($(f(self.$field)),+)
            }

            /// Construct a new point where each component is the result of
            /// applying the given operation to each pair of components of the
            /// given points.
            pub fn zip<F>(self, p2: Self, mut f: F) -> Self
            where F: FnMut($T, $T) -> $T
            {
                Self::new($(f(self.$field, p2.$field)),+)
            }

            /// Returns the squared distance.
            pub fn distance2(self, rhs: Self) -> $T {
                (rhs - self).magnitude2()
            }

            /// The distance between two values.
            pub fn distance(self, rhs: Self) -> $T {
                (rhs - self).magnitude()
            }

            /// Returns the result of linearly interpolating the vector
            /// towards `rhs` by the specified amount.
            pub fn lerp(self, rhs: Self, t: $T) -> Self {
                self + ((rhs - self) * t)
            }

            /// Returns the average position of all points in the slice.
            pub fn centroid(points: &[Self]) -> Self {
                let total_displacement = points
                    .iter()
                    .fold(Self::ORIGIN, |acc, p| acc + <$Point<$T> as Into<$Vector<$T>>>::into(*p));

                total_displacement / (points.len() as $T)
            }
        }

        impl ops::Neg for $Point<$T> {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new($(-self.$field),+)
            }
        }

        impl ops::Add<$Vector<$T>> for $Point<$T> {
            type Output = Self;

            fn add(self, rhs: $Vector<$T>) -> Self {
                Self::new($(self.$field + rhs.$field),+)
            }
        }

        impl<'a> ops::Add<&'a $Vector<$T>> for $Point<$T> {
            type Output = Self;

            fn add(self, rhs: &'a $Vector<$T>) -> Self {
                Self::new($(self.$field + rhs.$field),+)
            }
        }

        impl ops::AddAssign<$Vector<$T>> for $Point<$T> {
            fn add_assign(&mut self, rhs: $Vector<$T>) {
                $(self.$field += rhs.$field);+
            }
        }

        impl<'a> ops::AddAssign<&'a $Vector<$T>> for $Point<$T> {
            fn add_assign(&mut self, rhs: &'a $Vector<$T>) {
                $(self.$field += rhs.$field);+
            }
        }

        impl ops::Sub for $Point<$T> {
            type Output = $Vector<$T>;

            fn sub(self, rhs: Self) -> $Vector<$T> {
                $Vector::<$T>::new($(self.$field - rhs.$field),+)
            }
        }

        impl<'a> ops::Sub<&'a Self> for $Point<$T> {
            type Output = $Vector<$T>;

            fn sub(self, rhs: &'a Self) -> $Vector<$T> {
                $Vector::<$T>::new($(self.$field - rhs.$field),+)
            }
        }

        impl ops::Mul<$T> for $Point<$T> {
            type Output = Self;

            fn mul(self, rhs: $T) -> Self {
                Self::new($(self.$field * rhs),+)
            }
        }

        impl ops::Mul<$Point<$T>> for $T {
            type Output = $Point<$T>;

            fn mul(self, rhs: $Point<$T>) -> $Point<$T> {
                $Point::<$T>::new($(self * rhs.$field),+)
            }
        }

        impl<'a> ops::Mul<&'a $Point<$T>> for $T {
            type Output = $Point<$T>;

            fn mul(self, rhs: &'a $Point<$T>) -> $Point<$T> {
                $Point::<$T>::new($(self * rhs.$field),+)
            }
        }

        impl ops::MulAssign<$T> for $Point<$T> {
            fn mul_assign(&mut self, rhs: $T) {
                $(self.$field *= rhs);+
            }
        }

        impl ops::Div<$T> for $Point<$T> {
            type Output = Self;

            fn div(self, rhs: $T) -> Self {
                debug_assert!(rhs != $zero);
                Self::new($(self.$field / rhs),+)
            }
        }

        impl ops::DivAssign<$T> for $Point<$T> {
            fn div_assign(&mut self, rhs: $T) {
                debug_assert!(rhs != $zero);
                $(self.$field /= rhs);+
            }
        }

        impl ops::Rem<$T> for $Point<$T> {
            type Output = Self;

            fn rem(self, rhs: $T) -> Self {
                Self::new($(self.$field % rhs),+)
            }
        }

        impl ops::RemAssign<$T> for $Point<$T> {
            fn rem_assign(&mut self, rhs: $T) {
                $(self.$field %= rhs);+
            }
        }

        impl ops::Index<usize> for $Point<$T> {
            type Output = $T;

            fn index(&self, index: usize) -> &$T {
                match index {
                    $($index => &self.$field,)+
                    _ => panic!("out of range"),
                }
            }
        }

        impl ops::IndexMut<usize> for $Point<$T> {
            fn index_mut(&mut self, index: usize) -> &mut $T {
                match index {
                    $($index => &mut self.$field,)+
                    _ => panic!("out of range"),
                }
            }
        }

        impl From<$T> for $Point<$T> {
            fn from(value: $T) -> Self {
                Self { $($field: value),+}
            }
        }

        impl From<[$T; $n]> for $Point<$T> {
            fn from(value: [$T; $n]) -> Self {
                // We need to use a clone here because we can't pattern match on arrays yet
                Self::new($(value[$index]),+)
            }
        }

        impl<'a> From<&'a [$T; $n]> for $Point<$T> {
            fn from(value: &'a [$T; $n]) -> Self {
                // We need to use a clone here because we can't pattern match on arrays yet
                Self::new($(value[$index]),+)
            }
        }

        impl From<$Vector<$T>> for $Point<$T> {
            fn from(value: $Vector<$T>) -> Self {
                Self::ORIGIN + value
            }
        }

        impl<'a> From<&'a $Vector<$T>> for $Point<$T> {
            fn from(value: &'a $Vector<$T>) -> Self {
                Self::ORIGIN + value
            }
        }

        impl Into<[$T; $n]> for $Point<$T> {
            fn into(self) -> [$T; $n] {
               let Self { $($field),+ } = self;
               [$($field),+]
            }
        }
    };
    (@sum, $x:expr) => { $x };
    (@sum, $x:expr, $($t:expr),+) => {
        $x + impl_point!(@sum, $($t),+)
    };
}
