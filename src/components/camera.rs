use std::borrow::Cow;

use ::ffi;
use ::ffi::*;

use traits::Named;
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

impl<'a> Named<'a> for Camera<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> Camera<'a> {
    #[inline(always)]
    pub fn position(&self) -> AiVector3D { self.raw.position }

    #[inline(always)]
    pub fn look_at(&self) -> AiVector3D { self.raw.look_at }

    #[inline(always)]
    pub fn up(&self) -> AiVector3D { self.raw.up }

    #[inline(always)]
    pub fn hfov(&self) -> f32 { self.raw.hfov as f32 }

    #[inline(always)]
    pub fn znear(&self) -> f32 { self.raw.znear as f32 }

    #[inline(always)]
    pub fn zfar(&self) -> f32 { self.raw.zfar as f32 }

    #[inline(always)]
    pub fn aspect(&self) -> f32 { self.raw.aspect as f32 }
}