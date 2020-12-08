macro_rules! impl_vector {
    ($Vector:ident < $T:ty >, { $($field:ident : $index:expr),+ ; $n:expr }, { $zero:expr, $one:expr }, $abbr:ident) => {
        /// The short constructor.
        pub const fn $abbr($($field: $T),+) -> $Vector<$T> {
            $Vector::<$T>::new($($field),+)
        }

        impl $Vector<$T> {
            pub const ZERO:Self = Self::from_value($zero);

            /// Construct a new vector, using the provided values.
            pub const fn new($($field: $T),+) -> Self {
                Self { $($field),+ }
            }

            /// Construct a vector from a single value, replicating it.
            pub const fn from_value(value: $T) -> Self {
                Self { $($field: value),+ }
            }

            /// Perform the given operation on each field in the vector, returning a new point
            /// constructed from the operations.
            pub fn map<F>(self, mut f: F) -> Self
                where F: FnMut($T) -> $T
            {
                Self::new($(f(self.$field)),+)
            }

            /// Construct a new vector where each component is the result of
            /// applying the given operation to each pair of components of the
            /// given vectors.
            pub fn zip<F>(self, v: Self, mut f: F) -> Self
                where F: FnMut($T, $T) -> $T
            {
                Self::new ($(f(self.$field, v.$field)),+)
            }

            /// Vector dot (or inner) product.
            pub fn dot(self, rhs: Self) -> $T {
                impl_vector!(@sum, $(self.$field * rhs.$field),+)
            }

            /// Returns the squared magnitude.
            pub fn magnitude2(self) -> $T {
                self.dot(self)
            }

            /// The distance from the tail to the tip of the vector.
            pub fn magnitude(self) -> $T {
                self.magnitude2().sqrt()
            }

            /// Returns the squared distance.
            pub fn distance2(self, rhs: Self) -> $T {
                (rhs - self).magnitude2()
            }

            /// The distance between two values.
            pub fn distance(self, rhs: Self) -> $T {
                (rhs - self).magnitude()
            }

            /// Returns a vector with the same direction, but with a magnitude of `1`.
            pub fn normalize(self) -> Self {
                self.normalize_to($one)
            }

            /// Returns a vector with the same direction and a given magnitude.
            pub fn normalize_to(self, m: $T) -> Self {
                self * (m / self.magnitude())
            }

            /// Returns the angle between two vectors in radians.
            pub fn angle(self, rhs: Self) -> Rad<$T> {
                Rad::<$T>::acos(self.dot(rhs) / (self.magnitude() * rhs.magnitude()))
            }

            /// Returns the result of linearly interpolating the vector
            /// towards `rhs` by the specified amount.
            pub fn lerp(self, rhs: Self, t: $T) -> Self {
                self + ((rhs - self) * t)
            }

            /// Returns the
            /// [vector projection](https://en.wikipedia.org/wiki/Vector_projection)
            /// of the current inner space projected onto the supplied argument.
            pub fn project_on(self, rhs: Self) -> Self {
                rhs * (self.dot(rhs) / rhs.magnitude2())
            }
        }

        impl ops::Neg for $Vector<$T> {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new($(-self.$field),+)
            }
        }

        impl ops::Add for $Vector<$T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::new($(self.$field + rhs.$field),+)
            }
        }

        impl ops::Sub for $Vector<$T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::new($(self.$field - rhs.$field),+)
            }
        }

        impl ops::Mul for $Vector<$T> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self::new($(self.$field * rhs.$field),+)
            }
        }

        impl ops::Div for $Vector<$T> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                debug_assert!($(rhs.$field != $zero)&&+);
                Self::new($(self.$field / rhs.$field),+)
            }
        }

        impl ops::Rem for $Vector<$T> {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self {
                Self::new($(self.$field % rhs.$field),+)
            }
        }

        impl ops::AddAssign for $Vector<$T> {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field);+
            }
        }

        impl ops::SubAssign for $Vector<$T> {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$field -= rhs.$field);+
            }
        }

        impl ops::MulAssign for $Vector<$T> {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$field *= rhs.$field);+
            }
        }

        impl ops::DivAssign for $Vector<$T> {
            fn div_assign(&mut self, rhs: Self) {
                debug_assert!($(rhs.$field != $zero)&&+);
                $(self.$field /= rhs.$field);+
            }
        }

        impl ops::RemAssign for $Vector<$T> {
            fn rem_assign(&mut self, rhs: Self) {
                $(self.$field %= rhs.$field);+
            }
        }

        impl ops::Mul<$T> for $Vector<$T> {
            type Output = Self;

            fn mul(self, rhs: $T) -> Self {
                Self::new($(self.$field * rhs),+)
            }
        }

        impl ops::Div<$T> for $Vector<$T> {
            type Output = Self;

            fn div(self, rhs: $T) -> Self {
                debug_assert!(rhs != $zero);
                Self::new($(self.$field / rhs),+)
            }
        }

        impl ops::Rem<$T> for $Vector<$T> {
            type Output = Self;

            fn rem(self, rhs: $T) -> Self {
                Self::new($(self.$field % rhs),+)
            }
        }

        impl ops::MulAssign<$T> for $Vector<$T> {
            fn mul_assign(&mut self, rhs: $T) {
                $(self.$field *= rhs);+
            }
        }

        impl ops::DivAssign<$T> for $Vector<$T> {
            fn div_assign(&mut self, rhs: $T) {
                debug_assert!(rhs != $zero);
                $(self.$field /= rhs);+
            }
        }

        impl ops::RemAssign<$T> for $Vector<$T> {
            fn rem_assign(&mut self, rhs: $T) {
                $(self.$field %= rhs);+
            }
        }

        impl ops::Index<usize> for $Vector<$T> {
            type Output = $T;

            fn index(&self, index: usize) -> &$T {
                match index {
                    $($index => &self.$field,)+
                    _ => panic!("out of range"),
                }
            }
        }

        impl ops::IndexMut<usize> for $Vector<$T> {
            fn index_mut(&mut self, index: usize) -> &mut $T {
                match index {
                    $($index => &mut self.$field,)+
                    _ => panic!("out of range"),
                }
            }
        }

        impl ops::Mul<$Vector<$T>> for $T {
            type Output = $Vector<$T>;

            fn mul(self, rhs: $Vector<$T>) -> $Vector<$T> {
                $Vector::<$T>::new($(self * rhs.$field),+)
            }
        }

        impl iter::Sum for $Vector<$T> {
            fn sum<I>(iter: I) -> Self
            where I: Iterator<Item=Self>
            {
                iter.fold(Self::ZERO, ops::Add::add)
            }
        }

        impl Into<[$T; $n]> for $Vector<$T> {
            fn into(self) -> [$T; $n] {
               let Self { $($field),+ } = self;
               [$($field),+]
            }
        }

        impl From<[$T; $n]> for $Vector<$T> {
            fn from(arr: [$T; $n]) -> Self {
                // We need to use a clone here because we can't pattern match on arrays yet
                Self::new($(arr[$index]),+)
            }
        }

        impl<'a> From<&'a [$T; $n]> for $Vector<$T> {
            fn from(arr: &'a [$T; $n]) -> Self {
                // We need to use a clone here because we can't pattern match on arrays yet
                Self::new($(arr[$index]),+)
            }
        }
    };
    (@sum, $x:expr) => { $x };
    (@sum, $x:expr, $($t:expr),+) => {
        $x + impl_vector!(@sum, $($t),+)
    };
}

macro_rules! impl_vector1 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector1<$T> {
            pub const UNIT_X: Self = Self::new($one);
        }

        impl Into<($T,)> for Vector1<$T> {
            fn into(self) -> ($T,) {
                (self.x,)
            }
        }

        impl From<($T,)> for Vector1<$T> {
            fn from(tuple: ($T,)) -> Self {
                Self::new(tuple.0)
            }
        }

        impl<'a> From<&'a ($T,)> for Vector1<$T> {
            fn from(tuple: &'a ($T,)) -> Self {
                Self::new(tuple.0)
            }
        }

        impl fmt::Debug for Vector1<$T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({:?})", self.x)
            }
        }

        impl From<Vector2<$T>> for Vector1<$T> {
            fn from(v: Vector2<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl From<Vector3<$T>> for Vector1<$T> {
            fn from(v: Vector3<$T>) -> Self {
                Self::new(v.x)
            }
        }

        impl From<Vector4<$T>> for Vector1<$T> {
            fn from(v: Vector4<$T>) -> Self {
                Self::new(v.x)
            }
        }
    };
}

macro_rules! impl_vector2 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector2<$T> {
            pub const UNIT_X: Self = Self::new($one, $zero);
            pub const UNIT_Y: Self = Self::new($zero, $one);

            /// The perpendicular dot product of the vector and `rhs`.
            pub fn perp_dot(self, rhs: Self) -> $T {
                (self.x * rhs.y) - (self.y * rhs.x)
            }
        }

        impl Into<($T, $T)> for Vector2<$T> {
            fn into(self) -> ($T, $T) {
                (self.x, self.y)
            }
        }

        impl From<($T, $T)> for Vector2<$T> {
            fn from(tuple: ($T, $T)) -> Self {
                Self::new(tuple.0, tuple.1)
            }
        }

        impl<'a> From<&'a ($T, $T)> for Vector2<$T> {
            fn from(tuple: &'a ($T, $T)) -> Self {
                Self::new(tuple.0, tuple.1)
            }
        }

        impl fmt::Debug for Vector2<$T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({:?},{:?})", self.x, self.y)
            }
        }

        impl From<Vector1<$T>> for Vector2<$T> {
            fn from(v: Vector1<$T>) -> Self {
                Self::new(v.x, $zero)
            }
        }

        impl From<Vector3<$T>> for Vector2<$T> {
            fn from(v: Vector3<$T>) -> Self {
                Self::new(v.x, v.y)
            }
        }

        impl From<Vector4<$T>> for Vector2<$T> {
            fn from(v: Vector4<$T>) -> Self {
                Self::new(v.x, v.y)
            }
        }
    };
}

macro_rules! impl_vector3 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector3<$T> {
            pub const UNIT_X: Self = Self::new($one, $zero, $zero);
            pub const UNIT_Y: Self = Self::new($zero, $one, $zero);
            pub const UNIT_Z: Self = Self::new($zero, $zero, $one);

            /// Returns the cross product of the vector and `other`.
            pub fn cross(self, rhs: Self) -> Self {
                Self::new(
                    (self.y * rhs.z) - (self.z * rhs.y),
                    (self.z * rhs.x) - (self.x * rhs.z),
                    (self.x * rhs.y) - (self.y * rhs.x),
                )
            }
        }

        impl Into<($T, $T, $T)> for Vector3<$T> {
            fn into(self) -> ($T, $T, $T) {
                (self.x, self.y, self.z)
            }
        }

        impl From<($T, $T, $T)> for Vector3<$T> {
            fn from(tuple: ($T, $T, $T)) -> Self {
                Self::new(tuple.0, tuple.1, tuple.2)
            }
        }

        impl<'a> From<&'a ($T, $T, $T)> for Vector3<$T> {
            fn from(tuple: &'a ($T, $T, $T)) -> Self {
                Self::new(tuple.0, tuple.1, tuple.2)
            }
        }

        impl fmt::Debug for Vector3<$T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({:?},{:?},{:?})", self.x, self.y, self.z)
            }
        }

        impl From<Vector1<$T>> for Vector3<$T> {
            fn from(v: Vector1<$T>) -> Self {
                Self::new(v.x, $zero, $zero)
            }
        }

        impl From<Vector2<$T>> for Vector3<$T> {
            fn from(v: Vector2<$T>) -> Self {
                Self::new(v.x, v.y, $zero)
            }
        }

        impl From<Vector4<$T>> for Vector3<$T> {
            fn from(v: Vector4<$T>) -> Self {
                Self::new(v.x, v.y, v.z)
            }
        }
    };
}

macro_rules! impl_vector4 {
    ($T:ty, $one:expr, $zero:expr) => {
        impl Vector4<$T> {
            pub const UNIT_X: Self = Self::new($one, $zero, $zero, $zero);
            pub const UNIT_Y: Self = Self::new($zero, $one, $zero, $zero);
            pub const UNIT_Z: Self = Self::new($zero, $zero, $one, $zero);
            pub const UNIT_W: Self = Self::new($zero, $zero, $zero, $one);
        }

        impl Into<($T, $T, $T, $T)> for Vector4<$T> {
            fn into(self) -> ($T, $T, $T, $T) {
                (self.x, self.y, self.z, self.w)
            }
        }

        impl From<($T, $T, $T, $T)> for Vector4<$T> {
            fn from(tuple: ($T, $T, $T, $T)) -> Self {
                Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
            }
        }

        impl<'a> From<&'a ($T, $T, $T, $T)> for Vector4<$T> {
            fn from(tuple: &'a ($T, $T, $T, $T)) -> Self {
                Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
            }
        }

        impl fmt::Debug for Vector4<$T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({:?},{:?},{:?},{:?})", self.x, self.y, self.z, self.w)
            }
        }

        impl From<Vector1<$T>> for Vector4<$T> {
            fn from(v: Vector1<$T>) -> Self {
                Self::new(v.x, $zero, $zero, $zero)
            }
        }

        impl From<Vector2<$T>> for Vector4<$T> {
            fn from(v: Vector2<$T>) -> Self {
                Self::new(v.x, v.y, $zero, $zero)
            }
        }

        impl From<Vector3<$T>> for Vector4<$T> {
            fn from(v: Vector3<$T>) -> Self {
                Self::new(v.x, v.y, v.z, $zero)
            }
        }
    };
}
