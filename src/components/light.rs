use std::slice;
use std::marker::PhantomData;
use libc::c_uint;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use iterator::*;

pub struct Light<'a> {
    raw: &'a ffi::AiLight
}

impl<'a> AiIteratorAdapter<'a, Light<'a>> for Light<'a> {
    type Inner = *const ffi::AiLight;

    #[inline(always)]
    fn from(inner: &'a *const ffi::AiLight) -> Light<'a> {
        Light { raw: unsafe { inner.as_ref().expect("Light pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Light<'a> {
    //TODO
}