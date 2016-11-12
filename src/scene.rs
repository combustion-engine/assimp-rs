use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::slice;

use ffi::{self, AiVector3D, AiVector2D, AiMatrix3x3, AiMatrix4x4};
use error::*;
use postprocess::PostprocessEffect;
use components::*;

pub struct Scene<'a> {
    scene_ptr: *const ffi::AiScene,
    _lifetime: PhantomData<&'a ()>
}

/// The C API for Assimp is thread safe, so it's okay to move scenes between threads
unsafe impl<'a> Send for Scene<'a> {}

macro_rules! impl_scene_iterator {
    ($field:ident, $num_field:ident, $t:ty) => {
        pub fn $field(&self) -> Option<AiIterator<'a, $t>> {
            let scene: &ffi::AiScene = self.raw_scene();
            if scene.$field.is_null() || scene.$num_field == 0 { None } else {
                Some(AiIterator::from(unsafe { slice::from_raw_parts(scene.$field, scene.$num_field as usize) }))
            }
        }
    }
}

impl<'a> Scene<'a> {
    #[inline(always)]
    fn raw_scene(&self) -> &'a ffi::AiScene {
        unsafe { &*self.scene_ptr }
    }

    pub fn import<P: AsRef<Path>>(path: P, effects: Option<PostprocessEffect>) -> AiResult<Scene<'a>> {
        let c_path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let scene_ptr = unsafe {
            ffi::aiImportFile(c_path.as_ptr() as *const _, match effects {
                None => 0,
                Some(flags) => flags.bits()
            })
        };

        check_assimp_errors!(scene_ptr.is_null());

        Ok(Scene {
            scene_ptr: scene_ptr,
            _lifetime: PhantomData
        })
    }

    /// Apply postprocessing to the scene.
    ///
    /// Consumes the scene and returns a new one with the effects applied to it, that way
    /// no structure belonging to the original scene can maintain references to it.
    pub fn postprocess<'b>(self, effects: PostprocessEffect) -> AiResult<Scene<'b>> where 'a: 'b {
        let scene_ptr = unsafe {
            ffi::aiApplyPostProcessing(self.scene_ptr, effects.bits())
        };

        check_assimp_errors!(scene_ptr != self.scene_ptr);

        let new_scene = Scene {
            scene_ptr: scene_ptr,
            _lifetime: PhantomData
        };

        //Don't run the destructor on self, since we took ownership of the pointer
        mem::forget(self);

        Ok(new_scene)
    }

    impl_scene_iterator!(meshes, num_meshes, Mesh);
    impl_scene_iterator!(materials, num_materials, Material);
    impl_scene_iterator!(textures, num_textures, Texture);
    impl_scene_iterator!(lights, num_lights, Light);
    impl_scene_iterator!(cameras, num_cameras, Camera);
    impl_scene_iterator!(animations, num_animations, Animation);
}

impl<'a> Drop for Scene<'a> {
    fn drop(&mut self) {
        unsafe { ffi::aiReleaseImport(self.scene_ptr) }
    }
}