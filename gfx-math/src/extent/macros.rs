macro_rules! define_ext {
    ($Ext:ident, $Pt:ty) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $Ext(pub(crate) [$Pt; 2]);

        impl $Ext {
            #[inline]
            pub const fn new(min: $Pt, max: $Pt) -> Self {
                Self([min, max])
            }

            #[inline]
            pub const fn min(&self) -> &$Pt {
                &self.0[0]
            }

            #[inline]
            pub const fn max(&self) -> &$Pt {
                &self.0[1]
            }

            #[inline]
            pub fn set_min(&mut self, value: &$Pt) -> &mut Self {
                self.0[0] = *value;
                self
            }

            #[inline]
            pub fn set_max(&mut self, value: &$Pt) -> &mut Self {
                self.0[1] = *value;
                self
            }
        }
    };
}
