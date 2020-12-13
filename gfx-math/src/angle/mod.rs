#![allow(dead_code)]

use crate::number::ApproxEq;

#[macro_use]
mod macros;

define_angle!(Deg);
define_angle!(Rad);

#[cfg(feature = "use_f64")]
impl_angle!(Rad, f64, std::f64::consts::TAU, Deg);
#[cfg(feature = "use_f64")]
impl_rad!(f64);

#[cfg(feature = "use_f64")]
impl_angle!(Deg, f64, 360.0, Rad);
#[cfg(feature = "use_f64")]
impl_deg!(f64);

#[cfg(not(feature = "use_f64"))]
impl_angle!(Rad, f32, std::f32::consts::TAU, Deg);
#[cfg(not(feature = "use_f64"))]
impl_rad!(f32);

#[cfg(not(feature = "use_f64"))]
impl_angle!(Deg, f32, 360.0, Rad);
#[cfg(not(feature = "use_f64"))]
impl_deg!(f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let r = Rad::new(1.04);
        let d = Deg::new(100.3);

        assert_eq!(format!("{}", r), "1.04");
        assert_eq!(format!("{}", d), "100.3");
    }

    #[test]
    fn test_eq() {
        let r = Rad::new(1.04);
        let d = Deg::new(100.3);

        assert_eq!(r, Rad::new(1.0400000001));
        assert_ne!(r, Rad::new(1.041));

        assert_eq!(d, Deg::new(100.3000000001));
        assert_ne!(d, Deg::new(100.30001));
    }

    #[test]
    fn test_ops() {
        let r = Rad::quat();
        let d = Deg::quat();

        assert_eq!((r * -5.0).norm(), Rad::half() * 3.0 / 2.0);
        assert_eq!((d * -5.0).norm(), Deg::new(270.0));

        let r = Rad::quat();
        let d = Deg::new(90.0);

        assert_eq!((-r).to_norm(), Rad::full() * 3.0 / 4.0);
        assert_eq!((-d).to_norm(), Deg::full() * 3.0 / 4.0);

        assert_eq!(Rad::from(Deg::half()), Rad::half());
        assert_eq!(Deg::from(Rad::half()), Deg::half());
    }

    #[test]
    fn test_trigonometric() {
        let r = Rad::half();
        let d = Rad::half();

        assert!(r.sin().approx_eq(&0.0));
        assert!(d.sin().approx_eq(&0.0));
    }
}
