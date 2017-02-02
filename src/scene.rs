use libc::c_uint;
use std::ffi::CString;
use std::mem;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::slice;

use ffi;
use error::*;
use postprocess::PostprocessEffect;
use components::*;

pub struct Scene<'a> {
    scene_ptr: *const ffi::AiScene,
    path: PathBuf,
    _lifetime: PhantomData<&'a ()>
}

// The C API for Assimp is thread safe, so it's okay to move scenes between threads
unsafe impl<'a> Send for Scene<'a> {}

bitflags! {
    /// Scene bitflags
    pub flags SceneFlags: c_uint {
        const INCOMPLETE = ffi::SCENE_FLAG_INCOMPLETE,
        const VALIDATED = ffi::SCENE_FLAG_VALIDATED,
        const VALIDATION_WARNIGN = ffi::SCENE_FLAG_VALIDATION_WARNING,
        const NON_VERBOSE_FORMAT = ffi::SCENE_FLAG_NON_VERBOSE_FORMAT,
        const TERRAIN = ffi::SCENE_FLAG_TERRAIN,
        const SHARED = ffi::SCENE_FLAG_ALLOW_SHARED
    }
}

macro_rules! impl_scene_iterator {
    ($field:ident, $num_field:ident, $t:ident) => {
        pub fn $field(&self) -> Option<impl Iterator<Item = $t<'a>>> {
            let scene: &ffi::AiScene = self.raw_scene();
            if scene.$field.is_null() || scene.$num_field == 0 { None } else {
                Some(unsafe {
                    slice::from_raw_parts(scene.$field, scene.$num_field as usize)
                    .iter()
                    .map(|v| $t::from_raw(v))
                })
            }
        }
    }
}

impl<'a> Scene<'a> {
    #[inline(always)]
    fn raw_scene(&self) -> &'a ffi::AiScene {
        unsafe { &*self.scene_ptr }
    }

    #[inline]
    pub fn flags(&self) -> SceneFlags {
        SceneFlags::from_bits_truncate(self.raw_scene().flags)
    }

    /// Checks if the scene is concrete and complete.
    #[inline]
    pub fn valid(&self) -> bool {
        !(self.scene_ptr.is_null() ||
            self.flags().contains(INCOMPLETE) ||
            self.raw_scene().root_node.is_null())
    }

    #[inline]
    pub fn pathbuf(&self) -> &PathBuf {
        &self.path
    }

    #[inline]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn import<P: AsRef<Path>>(path: P, effects: Option<PostprocessEffect>) -> AiResult<Scene<'a>> {
        let path = path.as_ref();

        let c_path = CString::new(path.to_str().unwrap()).unwrap();

        let scene_ptr = unsafe {
            ffi::aiImportFile(c_path.as_ptr(), match effects {
                None => 0,
                Some(flags) => flags.bits()
            })
        };

        let scene = Scene {
            scene_ptr: scene_ptr,
            path: path.to_path_buf(),
            _lifetime: PhantomData
        };

        if !scene.valid() {
            check_assimp_errors!();

            return Err(AiError::InvalidScene);
        }

        Ok(scene)
    }

    /// Apply postprocessing to the scene.
    ///
    /// Consumes the scene and returns a new one with the effects applied to it, that way
    /// no structure belonging to the original scene can maintain references to it.
    pub fn postprocess<'b>(self, effects: PostprocessEffect) -> AiResult<Scene<'b>> where 'a: 'b {
        let scene_ptr = unsafe {
            ffi::aiApplyPostProcessing(self.scene_ptr, effects.bits())
        };

        let scene = Scene {
            scene_ptr: scene_ptr,
            path: self.path.clone(),
            _lifetime: PhantomData
        };

        if !scene.valid() {
            check_assimp_errors!();

            return Err(AiError::InvalidScene);
        }

        //Don't run the destructor on self, since we took ownership of the pointer
        mem::forget(self);

        Ok(scene)
    }


    impl_scene_iterator!(meshes, num_meshes, Mesh);
    impl_scene_iterator!(materials, num_materials, Material);
    impl_scene_iterator!(textures, num_textures, Texture);
    impl_scene_iterator!(lights, num_lights, Light);
    impl_scene_iterator!(cameras, num_cameras, Camera);
    impl_scene_iterator!(animations, num_animations, Animation);

    /// Get a specific mesh. The index is usually provided by some `Node`
    pub fn mesh(&self, index: usize) -> Option<Mesh<'a>> {
        let scene: &ffi::AiScene = self.raw_scene();

        if index >= scene.num_meshes as usize || scene.meshes.is_null() { None } else {
            //Exploit that `from` method used for the iterators
            Some(Mesh::from_raw(unsafe {
                &*scene.meshes.offset(index as isize)
            }))
        }
    }

    pub fn root(&self) -> Node<'a> {
        let scene: &ffi::AiScene = self.raw_scene();

        Node::from_raw(&scene.root_node)
    }
}

impl<'a> Drop for Scene<'a> {
    fn drop(&mut self) {
        unsafe { ffi::aiReleaseImport(self.scene_ptr) }
    }
}