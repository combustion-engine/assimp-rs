use std::slice;

use ::ffi;

use traits::FromRaw;

pub struct MaterialProperty<'a> {
    raw: &'a ffi::AiMaterialProperty,
}

impl<'a> FromRaw<'a, MaterialProperty<'a>> for MaterialProperty<'a> {
    type Raw = *const ffi::AiMaterialProperty;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> MaterialProperty<'a> {
        MaterialProperty { raw: unsafe { raw.as_ref().expect("MaterialProperty pointer provided by Asssimp was NULL") } }
    }
}

impl<'a> MaterialProperty<'a> {
    //TODO
}

pub struct Material<'a> {
    raw: &'a ffi::AiMaterial,
}

impl<'a> FromRaw<'a, Material<'a>> for Material<'a> {
    type Raw = *const ffi::AiMaterial;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Material<'a> {
        Material { raw: unsafe { raw.as_ref().expect("Material pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Material<'a> {
    pub fn properties(&self) -> Option<impl Iterator<Item = MaterialProperty<'a>>> {
        if self.raw.num_properties == 0 || self.raw.properties.is_null() || self.raw.num_allocated == 0 {
            None
        } else {
            Some(unsafe {
                slice::from_raw_parts(self.raw.properties, self.raw.num_properties as usize)
                    .iter()
                    .map(MaterialProperty::from_raw)
            })
        }
    }
}