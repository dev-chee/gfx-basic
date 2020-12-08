#![macro_use]

/// Generates a binary operator implementation for the permutations of by-ref and by-val
macro_rules! impl_operator {
    // When it is an unary operator
    (<$S:ident: $Constraint:ident> $Op:ident for $Lhs:ty {
        fn $op:ident($x:ident) -> $Output:ty {
            $body:expr 
        }
    }) => {
        impl<$S: $Constraint> $Op for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self) -> $Output {
                let $x = self;
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op for &'a $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self) -> $Output {
                let $x = self;
                $body
            }
        }
    };
    // When the right operand is a scalar
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ident> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
    // When the right operand is a compound type
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op<&'a $Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, 'b, $S: $Constraint> $Op<&'a $Rhs> for &'b $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
    // When the left operand is a scalar
    ($Op:ident<$Rhs:ident<$S:ident>> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl $Op<$Rhs<$S>> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: $Rhs<$S>) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a> $Op<&'a $Rhs<$S>> for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self, other: &'a $Rhs<$S>) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
}
