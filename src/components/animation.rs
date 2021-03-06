use libc::c_uint;

use std::slice;
use std::borrow::Cow;

use ::ffi;
use ::ffi::*;

use ::scene::*;

use traits::{Named, FromRaw};
use mesh::*;

pub struct VectorKey<'a> {
    raw: &'a ffi::AiVectorKey,
}

impl<'a> FromRaw<'a, VectorKey<'a>> for VectorKey<'a> {
    type Raw = ffi::AiVectorKey;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> VectorKey<'a> {
        VectorKey { raw: raw }
    }
}

impl<'a> VectorKey<'a> {
    #[inline(always)]
    pub fn time(&self) -> f64 {
        self.raw.time as f64
    }

    #[inline(always)]
    pub fn value(&self) -> AiVector3D {
        self.raw.value
    }
}

pub struct QuaternionKey<'a> {
    raw: &'a ffi::AiQuatKey,
}

impl<'a> FromRaw<'a, QuaternionKey<'a>> for QuaternionKey<'a> {
    type Raw = ffi::AiQuatKey;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> QuaternionKey<'a> {
        QuaternionKey { raw: raw }
    }
}

impl<'a> QuaternionKey<'a> {
    #[inline(always)]
    pub fn time(&self) -> f64 {
        self.raw.time as f64
    }

    #[inline(always)]
    pub fn value(&self) -> AiQuaternion {
        self.raw.value
    }
}

pub struct MeshKey<'a> {
    raw: &'a ffi::AiMeshKey,
}

impl<'a> FromRaw<'a, MeshKey<'a>> for MeshKey<'a> {
    type Raw = ffi::AiMeshKey;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> MeshKey<'a> {
        MeshKey { raw: raw }
    }
}

impl<'a> MeshKey<'a> {
    #[inline(always)]
    pub fn time(&self) -> f64 {
        self.raw.time as f64
    }

    #[inline(always)]
    pub fn value(&self) -> c_uint {
        self.raw.value
    }

    #[inline]
    pub fn mesh_from_scene(&self, scene: &'a Scene<'a>) -> Option<Mesh<'a>> {
        scene.mesh(self.value() as usize)
    }
}

pub struct NodeAnimation<'a> {
    raw: &'a ffi::AiNodeAnim,
}

impl<'a> Named<'a> for NodeAnimation<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> FromRaw<'a, NodeAnimation<'a>> for NodeAnimation<'a> {
    type Raw = *const ffi::AiNodeAnim;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> NodeAnimation<'a> {
        NodeAnimation { raw: unsafe { raw.as_ref().expect("NodeAnim pointer provided by Assimp was NULL") } }
    }
}

impl<'a> NodeAnimation<'a> {
    impl_optional_iterator!(position_keys, position_keys, num_position_keys, VectorKey);
    impl_optional_iterator!(rotation_keys, rotation_keys, num_rotation_keys, QuaternionKey);
    impl_optional_iterator!(scaling_keys, scaling_keys, num_scaling_keys, VectorKey);
}

pub struct MeshAnimation<'a> {
    raw: &'a ffi::AiMeshAnim,
}

impl<'a> Named<'a> for MeshAnimation<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> FromRaw<'a, MeshAnimation<'a>> for MeshAnimation<'a> {
    type Raw = *const ffi::AiMeshAnim;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> MeshAnimation<'a> {
        MeshAnimation { raw: unsafe { raw.as_ref().expect("MeshAnim pointer provided by Assimp was NULL") } }
    }
}

pub struct Animation<'a> {
    raw: &'a ffi::AiAnimation,
}

impl<'a> Named<'a> for Animation<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> FromRaw<'a, Animation<'a>> for Animation<'a> {
    type Raw = *const ffi::AiAnimation;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Animation<'a> {
        Animation { raw: unsafe { raw.as_ref().expect("Animation pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Animation<'a> {
    #[inline(always)]
    pub fn duration(&self) -> f64 {
        self.raw.duration as f64
    }

    #[inline(always)]
    pub fn ticks_per_second(&self) -> f64 {
        self.raw.ticks_per_second as f64
    }

    impl_optional_iterator!(node_channels, channels, num_channels, NodeAnimation);
    impl_optional_iterator!(mesh_channels, mesh_channels, num_mesh_channels, MeshAnimation);
}