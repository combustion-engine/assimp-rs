use std::borrow::Cow;

pub trait Named<'a> {
    fn name(&self) -> Cow<'a, str>;
}