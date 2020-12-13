macro_rules! define_circle {
    ($Circle:ident, $Pt:ty, $N:ty, $eps:expr) => {
        #[derive(Debug, Default, Copy, Clone)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $Circle {
            pub(crate) c: $Pt,
            pub(crate) r: $N,
        }
    };
}