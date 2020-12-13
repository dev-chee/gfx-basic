macro_rules! define_mat {
    ($Mat:ident, $Vec:ty, $n:expr) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $Mat(pub(crate) [$Vec; $n]);
    };
}
