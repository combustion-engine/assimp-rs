use std::slice;
use std::ffi::{CString, CStr};
use std::marker::PhantomData;
use libc::c_uint;
use std::cmp::min;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;
use ::error::*;

use iterator::*;

pub struct Texture<'a> {
    raw: &'a ffi::AiTexture,
}

impl<'a> AiIteratorAdapter<'a, Texture<'a>> for Texture<'a> {
    type Inner = *const ffi::AiTexture;

    #[inline(always)]
    fn from(inner: &'a *const ffi::AiTexture) -> Texture<'a> {
        Texture { raw: unsafe { inner.as_ref().expect("Texture pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Texture<'a> {
    #[inline]
    pub fn width(&self) -> u32 { self.raw.width as u32 }

    #[inline]
    pub fn height(&self) -> u32 { self.raw.height as u32 }

    pub fn check_format(&self, format: &str) -> AiResult<bool> {
        if format.len() > ffi::ARCH_FORMAT_HINT_LENGTH {
            Ok(false)
        } else {
            let c_str = try!(CString::new(format));

            Ok(unsafe {
                c_str.as_ref() == CStr::from_ptr(self.raw.arch_format_hint.as_ptr())
            })
        }
    }

    #[inline]
    pub fn data_raw(&self) -> &'a [ffi::AiTexel] {
        unsafe { slice::from_raw_parts(self.raw.data, (self.height() * self.width()) as usize) }
    }
}