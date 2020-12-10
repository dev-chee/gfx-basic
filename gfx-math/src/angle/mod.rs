#![allow(dead_code)]

mod deg;
mod rad;

pub use deg::Deg;
pub use rad::Rad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        let epsilon = 1.0e-6;

        println!("cf: {:.40}, f: {:.40}", 1.0 / Deg::half_cycle().0, Deg::frequency().0 * 2.0);
        println!("cq: {:.40}, f: {}", Rad::half_cycle().0 * 0.5, Rad::quat_cycle());

        assert!(Deg::from(90.0).approx_eq(Deg::half_cycle() * 0.5, epsilon.into()));
        assert!(Rad::from(std::f64::consts::PI * 0.5)
            .approx_eq(Rad::half_cycle() * 0.5, epsilon.into()));

        assert!((Deg::half_cycle() * 0.5 * -7.0)
            .normalize()
            .approx_eq(Deg::half_cycle() * 0.5, epsilon.into()));
        assert!((Rad::half_cycle() * 0.5 * -7.0)
            .normalize()
            .approx_eq(Rad::half_cycle() * 0.5, epsilon.into()));

        assert!((Deg::half_cycle() * 0.5 * -7.0)
            .normalize_signed()
            .approx_eq(-Deg::half_cycle() * 0.5, epsilon.into()));
        assert!((Rad::half_cycle() * 0.5 * -7.0)
            .normalize_signed()
            .approx_eq(-Rad::half_cycle() * 0.5, epsilon.into()));

        assert!((Deg::half_cycle() * 0.5)
            .lerp(-Deg::half_cycle() * 0.5, 0.5)
            .approx_eq(Deg::zero(), epsilon.into()));
        assert!((Rad::half_cycle() * 0.5)
            .lerp(-Rad::half_cycle() * 0.5, 0.5)
            .approx_eq(Rad::zero(), epsilon.into()));

        assert!((Deg::half_cycle() * 0.5).approx_eq(-(-Deg::half_cycle() * 0.5), epsilon.into()));
        assert!((Rad::half_cycle() * 0.5).approx_eq(-(-Rad::half_cycle() * 0.5), epsilon.into()));

        assert!((Deg::half_cycle() * 0.5 + Deg::half_cycle() * 0.5)
            .approx_eq(Deg::half_cycle(), epsilon.into()));
        assert!((Rad::half_cycle() * 0.5 + Rad::half_cycle() * 0.5)
            .approx_eq(Rad::half_cycle(), epsilon.into()));

        assert!((Deg::half_cycle() - Deg::half_cycle() * 0.5)
            .approx_eq(Deg::half_cycle() * 0.5, epsilon.into()));
        assert!((Rad::half_cycle() - Rad::half_cycle() * 0.5)
            .approx_eq(Rad::half_cycle() * 0.5, epsilon.into()));

        assert!((Deg::half_cycle() * 2.0).approx_eq(Deg::cycle(), epsilon.into()));
        assert!((Rad::half_cycle() * 2.0).approx_eq(Rad::cycle(), epsilon.into()));

        assert!((Deg::cycle() / 2.0).approx_eq(Deg::half_cycle(), epsilon.into()));
        assert!((Rad::cycle() / 2.0).approx_eq(Rad::half_cycle(), epsilon.into()));

        assert!((Deg::cycle() % Deg::half_cycle() * 0.5).approx_eq(Deg::zero(), epsilon.into()));
        assert!((Rad::cycle() % Rad::half_cycle() * 0.5).approx_eq(Rad::zero(), epsilon.into()));

        let mut deg = Deg::from(90.0);
        let mut rad = Rad::from(std::f64::consts::PI * 0.5);
        deg += Deg::half_cycle() * 0.5;
        rad += Rad::half_cycle() * 0.5;

        assert!(deg.approx_eq(Deg::half_cycle(), epsilon.into()));
        assert!(rad.approx_eq(Rad::half_cycle(), epsilon.into()));

        let mut deg = Deg::from(180.0);
        let mut rad = Rad::from(std::f64::consts::PI);
        deg -= Deg::half_cycle() * 0.5;
        rad -= Rad::half_cycle() * 0.5;

        assert!(deg.approx_eq(Deg::half_cycle() * 0.5, epsilon.into()));
        assert!(rad.approx_eq(Rad::half_cycle() * 0.5, epsilon.into()));

        let mut deg = Deg::from(90.0);
        let mut rad = Rad::from(std::f64::consts::PI * 0.5);
        deg *= 2.0;
        rad *= 2.0;

        assert!(deg.approx_eq(Deg::cycle() * 0.5, epsilon.into()));
        assert!(rad.approx_eq(Rad::cycle() * 0.5, epsilon.into()));

        let mut deg = Deg::from(180.0);
        let mut rad = Rad::from(std::f64::consts::PI);
        deg /= 2.0;
        rad /= 2.0;

        assert!(deg.approx_eq(Deg::half_cycle() * 0.5, epsilon.into()));
        assert!(rad.approx_eq(Rad::half_cycle() * 0.5, epsilon.into()));
    }
}
