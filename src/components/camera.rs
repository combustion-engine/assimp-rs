use std::slice;
use std::marker::PhantomData;
use libc::c_uint;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use iterator::*;

pub struct Camera<'a> {
    raw: &'a ffi::AiCamera
}

impl<'a> AiIteratorAdapter<'a, Camera<'a>> for Camera<'a> {
    type Inner = *const ffi::AiCamera;

    #[inline(always)]
    fn from(inner: &'a *const ffi::AiCamera) -> Camera<'a> {
        Camera { raw: unsafe { inner.as_ref().expect("Camera pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Camera<'a> {
    //TODO
}