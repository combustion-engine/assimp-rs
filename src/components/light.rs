use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::borrow::Cow;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use traits::{Named, FromRaw};

enum_from_primitive! {
    #[repr(u32)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum LightKind {
        Directional = ffi::LIGHT_SOURCE_DIRECTIONAL,
        Point = ffi::LIGHT_SOURCE_POINT,
        Spotlight = ffi::LIGHT_SOURCE_SPOT,
        Ambient = ffi::LIGHT_SOURCE_AMBIENT,
        Area = ffi::LIGHT_SOURCE_AREA,
        Undefined = ffi::LIGHT_SOURCE_UNDEFINED
    }
}

pub struct Light<'a> {
    raw: &'a ffi::AiLight
}

impl<'a> FromRaw<'a, Light<'a>> for Light<'a> {
    type Raw = *const ffi::AiLight;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Light<'a> {
        Light { raw: unsafe { raw.as_ref().expect("Light pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Named<'a> for Light<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> Light<'a> {
    #[inline]
    pub fn kind(&self) -> Option<LightKind> {
        LightKind::from_u32(self.raw.kind as u32)
    }

    #[inline(always)]
    pub fn position(&self) -> AiVector3D { self.raw.position }

    #[inline(always)]
    pub fn direction(&self) -> AiVector3D { self.raw.direction }

    #[inline(always)]
    pub fn up(&self) -> AiVector3D { self.raw.up }

    #[inline]
    pub fn attenuation(&self) -> (f32, f32, f32) {
        (
            self.raw.attenuation_constant as f32,
            self.raw.attenuation_linear as f32,
            self.raw.attenuation_quadratic as f32
        )
    }

    #[inline(always)]
    pub fn diffuse(&self) -> AiColor3D { self.raw.diffuse_color }

    #[inline(always)]
    pub fn specular(&self) -> AiColor3D { self.raw.specular_color }

    #[inline(always)]
    pub fn ambient(&self) -> AiColor3D { self.raw.ambient_color }

    #[inline]
    pub fn cone(&self) -> (f32, f32) {
        (
            self.raw.inner_angle as f32,
            self.raw.outer_angle as f32
        )
    }

    #[inline(always)]
    pub fn size(&self) -> AiVector2D { self.raw.size }
}

impl<'a> Debug for Light<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Light {{name: `{}`, kind: {:?}, position: {:?}, direction: {:?}}}", self.name(), self.kind(), self.position(), self.direction())
    }
}