use std::borrow::Cow;

pub trait Named<'a> {
    fn name(&self) -> Cow<'a, str>;
}

pub trait FromRaw<'a, T> {
    type Raw;

    fn from_raw(raw: &'a Self::Raw) -> T;
}