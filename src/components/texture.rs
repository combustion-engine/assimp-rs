use std::slice;
use std::ffi::{CString, CStr};

use traits::FromRaw;
use ::ffi;
use ::error::*;

pub struct Texture<'a> {
    raw: &'a ffi::AiTexture,
}

impl<'a> FromRaw<'a, Texture<'a>> for Texture<'a> {
    type Raw = *const ffi::AiTexture;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Texture<'a> {
        Texture { raw: unsafe { raw.as_ref().expect("Texture pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Texture<'a> {
    #[inline]
    pub fn width(&self) -> u32 {
        self.raw.width as u32
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.raw.height as u32
    }

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