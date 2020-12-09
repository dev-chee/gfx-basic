#![allow(dead_code)]

#[macro_use]
mod macros;

mod deg;
mod rad;

pub use deg::Deg;
pub use rad::Rad;

#[cfg(test)]
mod tests {
    type Deg = super::Deg<f32>;
    type Rad = super::Rad<f32>;

    #[test]
    fn test_angle() {
        let epsilon = 1.0e-6_f32;

        assert!(Deg::from(90.0).approx_eq(Deg::QUART_TURN, epsilon.into()));
        assert!(Rad::from(std::f32::consts::PI * 0.5).approx_eq(Rad::QUART_TURN, epsilon.into()));

        assert!((Deg::QUART_TURN * -7.0)
            .normalize()
            .approx_eq(Deg::QUART_TURN, epsilon.into()));
        assert!((Rad::QUART_TURN * -7.0)
            .normalize()
            .approx_eq(Rad::QUART_TURN, epsilon.into()));

        assert!((Deg::QUART_TURN * -7.0)
            .normalize_signed()
            .approx_eq(-Deg::QUART_TURN, epsilon.into()));
        assert!((Rad::QUART_TURN * -7.0)
            .normalize_signed()
            .approx_eq(-Rad::QUART_TURN, epsilon.into()));

        assert!((Deg::QUART_TURN)
            .lerp(-Deg::QUART_TURN, 0.5)
            .approx_eq(Deg::ZERO_TURN, epsilon.into()));
        assert!((Rad::QUART_TURN)
            .lerp(-Rad::QUART_TURN, 0.5)
            .approx_eq(Rad::ZERO_TURN, epsilon.into()));

        assert!((Deg::QUART_TURN).approx_eq(-(-Deg::QUART_TURN), epsilon.into()));
        assert!((Rad::QUART_TURN).approx_eq(-(-Rad::QUART_TURN), epsilon.into()));

        assert!((Deg::QUART_TURN + Deg::QUART_TURN).approx_eq(Deg::HALF_TURN, epsilon.into()));
        assert!((Rad::QUART_TURN + Rad::QUART_TURN).approx_eq(Rad::HALF_TURN, epsilon.into()));

        assert!((Deg::HALF_TURN - Deg::QUART_TURN).approx_eq(Deg::QUART_TURN, epsilon.into()));
        assert!((Rad::HALF_TURN - Rad::QUART_TURN).approx_eq(Rad::QUART_TURN, epsilon.into()));

        assert!((Deg::HALF_TURN * 2.0).approx_eq(Deg::FULL_TURN, epsilon.into()));
        assert!((Rad::HALF_TURN * 2.0).approx_eq(Rad::FULL_TURN, epsilon.into()));

        assert!((Deg::FULL_TURN / 2.0).approx_eq(Deg::HALF_TURN, epsilon.into()));
        assert!((Rad::FULL_TURN / 2.0).approx_eq(Rad::HALF_TURN, epsilon.into()));

        assert!((Deg::FULL_TURN % Deg::QUART_TURN).approx_eq(Deg::ZERO_TURN, epsilon.into()));
        assert!((Rad::FULL_TURN % Rad::QUART_TURN).approx_eq(Rad::ZERO_TURN, epsilon.into()));

        let mut deg = Deg::from(90.0);
        let mut rad = Rad::from(std::f32::consts::PI * 0.5);
        deg += Deg::QUART_TURN;
        rad += Rad::QUART_TURN;

        assert!(deg.approx_eq(Deg::HALF_TURN, epsilon.into()));
        assert!(rad.approx_eq(Rad::HALF_TURN, epsilon.into()));

        let mut deg = Deg::from(180.0);
        let mut rad = Rad::from(std::f32::consts::PI);
        deg -= Deg::QUART_TURN;
        rad -= Rad::QUART_TURN;

        assert!(deg.approx_eq(Deg::QUART_TURN, epsilon.into()));
        assert!(rad.approx_eq(Rad::QUART_TURN, epsilon.into()));

        let mut deg = Deg::from(90.0);
        let mut rad = Rad::from(std::f32::consts::PI * 0.5);
        deg *= 2.0;
        rad *= 2.0;

        assert!(deg.approx_eq(Deg::HALF_TURN, epsilon.into()));
        assert!(rad.approx_eq(Rad::HALF_TURN, epsilon.into()));

        let mut deg = Deg::from(180.0);
        let mut rad = Rad::from(std::f32::consts::PI);
        deg /= 2.0;
        rad /= 2.0;

        assert!(deg.approx_eq(Deg::QUART_TURN, epsilon.into()));
        assert!(rad.approx_eq(Rad::QUART_TURN, epsilon.into()));
    }
}
