#![allow(dead_code)]

#[macro_use]
mod macros;

mod deg;
mod rad;

pub use deg::Deg;
pub use rad::Rad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(
            (Rad::<f64>::FULL_TURN * 1.2).normalize(),
            Rad::<f64>::FULL_TURN * 0.2
        );
        assert_eq!(
            (Rad::<f64>::FULL_TURN * -1.2).normalize(),
            Rad::<f64>::FULL_TURN * 0.8
        );

        assert_eq!(
            (Rad::<f64>::FULL_TURN * 1.2).normalize_signed(),
            Rad::<f64>::FULL_TURN * 0.2 - Rad::<f64>::HALF_TURN
        );
        assert_eq!(
            (Rad::<f64>::FULL_TURN * -1.2).normalize_signed(),
            Rad::<f64>::FULL_TURN * 0.8 - Rad::<f64>::HALF_TURN
        );

        assert_eq!(
            (Deg::<f64>::FULL_TURN * 1.2).normalize(),
            Deg::<f64>::FULL_TURN * 0.2
        );
        assert_eq!(
            (Deg::<f64>::FULL_TURN * -1.2).normalize(),
            Deg::<f64>::FULL_TURN * 0.8
        );

        assert_eq!(
            (Deg::<f64>::FULL_TURN * 1.2).normalize_signed(),
            Deg::<f64>::FULL_TURN * 0.2 - Deg::<f64>::HALF_TURN
        );
        assert_eq!(
            (Deg::<f64>::FULL_TURN * -1.2).normalize_signed(),
            Deg::<f64>::FULL_TURN * 0.8 - Deg::<f64>::HALF_TURN
        );
    }

    #[test]
    fn test_lerp() {
        assert_eq!(
            Rad::<f64>::ZERO_TURN.lerp(Rad::<f64>::HALF_TURN, 0.5),
            Rad::<f64>::QUART_TURN
        );
        assert_eq!(
            Rad::<f64>::ZERO_TURN.lerp(-Rad::<f64>::HALF_TURN, 0.5),
            -Rad::<f64>::QUART_TURN
        );

        assert_eq!(
            Rad::<f64>::QUART_TURN.lerp(Rad::<f64>::HALF_TURN, 0.5),
            Rad::<f64>::QUART_TURN * 1.5
        );
        assert_eq!(
            Rad::<f64>::QUART_TURN.lerp(-Rad::<f64>::HALF_TURN, 0.5),
            -Rad::<f64>::QUART_TURN * 0.5
        );

        assert_eq!(
            Deg::<f64>::ZERO_TURN.lerp(Deg::<f64>::HALF_TURN, 0.5),
            Deg::<f64>::QUART_TURN
        );
        assert_eq!(
            Deg::<f64>::ZERO_TURN.lerp(-Deg::<f64>::HALF_TURN, 0.5),
            -Deg::<f64>::QUART_TURN
        );

        assert_eq!(
            Deg::<f64>::QUART_TURN.lerp(Deg::<f64>::HALF_TURN, 0.5),
            Deg::<f64>::QUART_TURN * 1.5
        );
        assert_eq!(
            Deg::<f64>::QUART_TURN.lerp(-Deg::<f64>::HALF_TURN, 0.5),
            -Deg::<f64>::QUART_TURN * 0.5
        );
    }

    #[test]
    fn test_cos_sin() {
        assert_eq!(Rad(0.0_f32).sin(), 0.0);
        assert_eq!(Rad::<f32>::QUART_TURN.sin(), 1.0);

        assert_eq!(Deg::<f32>::QUART_TURN.sin(), 1.0);
        assert!(Deg::<f32>::HALF_TURN.sin() - 0.0 < 0.0001);
    }

    #[test]
    fn test_impl_from() {
        assert_eq!(Deg::from(Rad(std::f32::consts::PI / 2.0_f32)), Deg(90_f32));
        assert_eq!(Deg::from(Rad(std::f64::consts::PI / 2.0_f64)), Deg(90_f64));

        assert_eq!(Rad::from(Deg(180.0_f32)), Rad(std::f32::consts::PI));
        assert_eq!(Rad::from(Deg(180.0_f64)), Rad(std::f64::consts::PI));
    }
}
