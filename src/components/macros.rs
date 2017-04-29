macro_rules! impl_ref_raw {
    ($t:ty) => {
        #[inline(always)]
        fn ref_raw(&self) -> &'a $t {
            unsafe { &*(self.raw) }
        }
    }
}

macro_rules! impl_optional_iterator {
    ($name:ident, $field:ident, $num_field:ident, $ret:ident $(, {$(#[$attr:meta])*})*) => {
        $($(#[$attr])*)*
        pub fn $name(&self) -> Option<Box<Iterator<Item = $ret<'a>>>> {
            if self.raw.$num_field == 0 || self.raw.$field.is_null() { None } else {
                Some(Box::new(unsafe{
                    slice::from_raw_parts(self.raw.$field, self.raw.$num_field as usize)
                    .iter()
                    .map(|v| $ret::from_raw(v))
                }))
            }
        }
    }
}

macro_rules! impl_optional_slice {
    ($name:ident, $field:ident, $num_field:ident, $ret:ident $(, {$(#[$attr:meta])*})*) => {
        $($(#[$attr])*)*
        pub fn $name(&self) -> Option<&'a [$ret]> {
            if self.raw.$num_field == 0 || self.raw.$field.is_null() { None } else {
                Some(unsafe {
                    slice::from_raw_parts(self.raw.$field, self.raw.$num_field as usize)
                })
            }
        }
    }
}