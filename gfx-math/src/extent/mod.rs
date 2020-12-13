use crate::point::{Pt2, Pt3};

#[macro_use]
mod macros;

define_ext!(Ext2, Pt2);
define_ext!(Ext3, Pt3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        // assert!(Ext2::new(Pt2::origin(), Pt2::from(5.0)), Ext2::)
    }
}
