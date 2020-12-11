#![allow(dead_code)]

use std::fmt;

define_vec!(Vec2, f64, 2, { x:0, y:1 });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        assert_eq!(Vec2::new(0.0, 0.0), Vec2::zero());
        assert_eq!(Vec2::new(1.0, 1.0).x(), 1.0);
        assert_eq!(Vec2::new(1.0, 1.0).y(), 1.0);
    }
}
