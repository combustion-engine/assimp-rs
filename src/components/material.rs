use std::slice;
use std::marker::PhantomData;
use libc::c_uint;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use iterator::*;

pub struct MaterialProperty<'a> {
    raw: &'a ffi::AiMaterialProperty,
}

impl<'a> AiIteratorAdapter<'a, MaterialProperty<'a>> for MaterialProperty<'a> {
    type Inner = *const ffi::AiMaterialProperty;

    #[inline(always)]
    fn from(inner: &'a *const ffi::AiMaterialProperty) -> MaterialProperty<'a> {
        MaterialProperty { raw: unsafe { inner.as_ref().expect("MaterialProperty pointer provided by Asssimp was NULL") } }
    }
}

impl<'a> MaterialProperty<'a> {
    //TODO
}

pub struct Material<'a> {
    raw: &'a ffi::AiMaterial,
}

impl<'a> AiIteratorAdapter<'a, Material<'a>> for Material<'a> {
    type Inner = *const ffi::AiMaterial;

    #[inline(always)]
    fn from(inner: &'a *const ffi::AiMaterial) -> Material<'a> {
        Material { raw: unsafe { inner.as_ref().expect("Material pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Material<'a> {
    pub fn properties(&self) -> Option<AiIterator<'a, MaterialProperty<'a>>> {
        if self.raw.num_properties == 0 || self.raw.properties.is_null() || self.raw.num_allocated == 0 {
            None
        } else {
            Some(AiIterator::from(unsafe { slice::from_raw_parts(self.raw.properties, self.raw.num_properties as usize) }))
        }
    }
}