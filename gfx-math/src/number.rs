pub trait ApproxEq {
    fn epsilon() -> Self;

    fn approx_eq(&self, rhs: &Self) -> bool;
}

impl ApproxEq for f32 {
    #[inline]
    fn epsilon() -> Self {
        1e-6
    }

    #[inline]
    fn approx_eq(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < Self::epsilon()
    }
}

impl ApproxEq for f64 {
    #[inline]
    fn epsilon() -> Self {
        1e-6
    }

    #[inline]
    fn approx_eq(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < Self::epsilon()
    }
}

pub trait Cast {
    type Target;

    fn cast(&self) -> Self::Target;
}

impl Cast for f32 {
    type Target = f64;

    fn cast(&self) -> Self::Target {
        *self as f64
    }
}

impl Cast for f64 {
    type Target = f32;

    fn cast(&self) -> f32 {
        *self as f32
    }
}
