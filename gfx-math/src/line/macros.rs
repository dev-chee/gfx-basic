macro_rules! define_line {
    ($Line:ident, $Pt:ty, $Vec:ty) => {
        #[derive(Debug, Default, Copy, Clone)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $Line {
            pub(crate) pt: $Pt,
            pub(crate) dir: $Vec,
        }

        impl $Line {
            pub const fn new(pt: &$Pt, dir: &$Vec) -> Self {
                Self { pt: *pt, dir: *dir }
            }

            pub fn from_pt(p: $Pt, q: $Pt) -> Self {
                Self {
                    pt: p,
                    dir: *(q - p).to_norm(),
                }
            }
        }
    };
}
