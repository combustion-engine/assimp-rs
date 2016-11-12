use std::slice;
use std::marker::PhantomData;
use libc::c_uint;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use iterator::*;

pub struct Animation<'a> {
    raw: &'a ffi::AiAnimation
}

impl<'a> AiIteratorAdapter<'a, Animation<'a>> for Animation<'a> {
    type Inner = *const ffi::AiAnimation;

    #[inline(always)]
    fn from(inner: &'a *const ffi::AiAnimation) -> Animation<'a> {
        Animation { raw: unsafe { inner.as_ref().expect("Animation pointer provided by Assimp was NULL") } }
    }
}