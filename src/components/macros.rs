macro_rules! impl_ref_raw {
    ($t:ty) => {
        #[inline(always)]
        fn ref_raw(&self) -> &'a $t {
            unsafe { &*(self.raw) }
        }
    }
}