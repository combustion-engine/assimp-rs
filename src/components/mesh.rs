use std::slice;
use libc::c_uint;
use std::borrow::Cow;
use std::sync::Arc;

use lazy;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use traits::{Named, FromRaw};

pub struct VertexWeight<'a> {
    raw: &'a ffi::AiVertexWeight
}

impl<'a> FromRaw<'a, VertexWeight<'a>> for VertexWeight<'a> {
    type Raw = ffi::AiVertexWeight;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> VertexWeight<'a> {
        VertexWeight { raw: raw }
    }
}

impl<'a> VertexWeight<'a> {
    #[inline(always)]
    pub fn id(&self) -> u32 {
        self.raw.vertex_id as u32
    }

    #[inline(always)]
    pub fn weight(&self) -> f32 {
        self.raw.weight as f32
    }
}

pub struct Bone<'a> {
    raw: &'a ffi::AiBone
}

impl<'a> FromRaw<'a, Bone<'a>> for Bone<'a> {
    type Raw = ffi::AiBone;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Bone<'a> {
        Bone { raw: raw }
    }
}

impl<'a> Named<'a> for Bone<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> Bone<'a> {
    /// Returns a slice to the underlying C PODs for more efficient access than running through
    /// the iterator
    #[inline(always)]
    pub fn raw_weights(&self) -> &'a [ffi::AiVertexWeight] {
        unsafe { slice::from_raw_parts(self.raw.weights, self.raw.num_weights as usize) }
    }

    /// Returns an iterator to the vertex weights
    #[inline]
    pub fn weights(&self) -> impl Iterator<Item = VertexWeight<'a>> {
        self.raw_weights().iter().map(VertexWeight::from_raw)
    }

    /// Get a reference to the offset matrix of the bone
    #[inline(always)]
    pub fn offset(&self) -> &AiMatrix4x4 {
        &self.raw.offset_matrix
    }
}

//////////////////

pub struct Face<'a> {
    raw: &'a ffi::AiFace
}

impl<'a> FromRaw<'a, Face<'a>> for Face<'a> {
    type Raw = ffi::AiFace;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Face<'a> {
        Face { raw: raw }
    }
}

impl<'a> Face<'a> {
    #[inline(always)]
    pub fn indices(&self) -> &'a [c_uint] {
        unsafe { slice::from_raw_parts(self.raw.indices, self.raw.num_indices as usize) }
    }
}

//////////////////

enum_from_primitive! {
    #[repr(u32)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum PrimitiveType {
        Point = ffi::PRIMITIVE_TYPE_POINT,
        Line = ffi::PRIMITIVE_TYPE_LINE,
        Triangle = ffi::PRIMITIVE_TYPE_TRIANGLE,
        Polygon = ffi::PRIMITIVE_TYPE_POLYGON
    }
}

pub struct Mesh<'a> {
    raw: &'a ffi::AiMesh,
    indices: Arc<lazy::Lazy<Vec<c_uint>>>
}

impl<'a> Clone for Mesh<'a> {
    fn clone(&self) -> Mesh<'a> {
        Mesh {
            raw: self.raw,
            indices: self.indices.clone()
        }
    }
}

impl<'a> FromRaw<'a, Mesh<'a>> for Mesh<'a> {
    type Raw = *const ffi::AiMesh;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Mesh<'a> {
        Mesh {
            raw: unsafe { raw.as_ref().expect("Mesh pointer provided by Assimp was NULL") },
            indices: Arc::default()
        }
    }
}

impl<'a> Named<'a> for Mesh<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> Mesh<'a> {
    /// Gets the primitive type of the mesh.
    ///
    /// Returns `None` if the primitive type was not in the `PrimitiveType` enum,
    /// which shouldn't happen unless Assimp does something really weird.
    #[inline(always)]
    pub fn primitive_type(&self) -> Option<PrimitiveType> {
        PrimitiveType::from_u32(self.raw.primitive_type as u32)
    }

    impl_optional_iterator!(faces, faces, num_faces, Face, {
        /// Returns an iterator to all the faces in the mesh
    });

    impl_optional_slice!(vertices, vertices, num_vertices, AiVector3D);
    impl_optional_slice!(normals, normals, num_vertices, AiVector3D);
    impl_optional_slice!(tangents, tangents, num_vertices, AiVector3D);
    impl_optional_slice!(bitangents, bitangents, num_vertices, AiVector3D);

    /// Gets the number of active UV(W) coordinate systems for the mesh. Meshes can have more than one.
    pub fn uv_channels(&self) -> usize {
        self.raw.num_uvs.iter().filter(|dim| **dim > 0).count()
    }

    /// Gets a specific UV(W) channel and the number of dimensions it contains.
    pub fn uv_channel(&self, index: usize) -> Option<(u32, &'a [AiVector3D])> {
        if index < ffi::MAX_NUMBER_OF_TEXTURECOORDS as usize {
            if self.raw.num_uvs[index] == 0 || self.raw.texcoords[index].is_null() { None } else {
                Some((
                    self.raw.num_uvs[index] as u32,
                    unsafe { slice::from_raw_parts(self.raw.texcoords[index], self.raw.num_vertices as usize) },
                ))
            }
        } else {
            None
        }
    }

    /// Counts the number of indices for the mesh. This is NOT zero-cost
    pub fn count_indices(&self) -> Option<usize> {
        if let Some(indices) = self.indices() {
            Some(indices.len())
        } else {
            None
        }
    }

    /// Accumulates the indices for every face in the mesh. This is NOT zero-cost
    pub fn indices(&self) -> Option<&Vec<c_uint>> {
        if let Some(indices) = self.indices.get_maybe() {
            Some(indices)
        } else if let Some(faces) = self.faces() {
            let mut indices = Vec::new();

            for ref face in faces {
                indices.extend_from_slice(face.indices());
            }

            unsafe { self.indices.set(indices); }

            self.indices()
        } else {
            None
        }
    }

    /// Get the index of the material for this mesh
    #[inline(always)]
    pub fn material_index(&self) -> u32 {
        self.raw.material_index as u32
    }
}
