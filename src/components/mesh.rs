use std::slice;
use std::marker::PhantomData;
use libc::c_uint;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use iterator::*;

pub struct VertexWeight<'a> {
    raw: &'a ffi::AiVertexWeight
}

impl<'a> AiIteratorAdapter<'a, VertexWeight<'a>> for VertexWeight<'a> {
    type Inner = ffi::AiVertexWeight;

    #[inline(always)]
    fn from(inner: &'a ffi::AiVertexWeight) -> VertexWeight<'a> {
        VertexWeight { raw: inner }
    }
}

impl<'a> VertexWeight<'a> {
    #[inline(always)]
    pub fn id(&self) -> u32 { self.raw.vertex_id as u32 }

    #[inline(always)]
    pub fn weight(&self) -> f32 { self.raw.weight as f32 }
}

pub struct Bone<'a> {
    raw: &'a ffi::AiBone
}

impl<'a> AiIteratorAdapter<'a, Bone<'a>> for Bone<'a> {
    type Inner = ffi::AiBone;

    #[inline(always)]
    fn from(inner: &'a ffi::AiBone) -> Bone<'a> {
        Bone { raw: inner }
    }
}

impl<'a> Bone<'a> {
    /// Get the name of the bone
    pub fn name(&self) -> String {
        return self.raw.name.clone().into()
    }

    /// Returns a slice to the underlying C PODs for more efficient access than running through
    /// the iterator
    #[inline(always)]
    pub fn raw_weights(&self) -> &'a [ffi::AiVertexWeight] {
        unsafe { slice::from_raw_parts(self.raw.weights, self.raw.num_weights as usize) }
    }

    /// Returns an iterator to the vertex weights
    #[inline]
    pub fn weights(&self) -> AiIterator<'a, VertexWeight<'a>> {
        AiIterator::from(self.raw_weights().iter())
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

impl<'a> AiIteratorAdapter<'a, Face<'a>> for Face<'a> {
    type Inner = ffi::AiFace;

    #[inline(always)]
    fn from(inner: &'a ffi::AiFace) -> Face<'a> {
        Face { raw: inner }
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
    raw: &'a ffi::AiMesh
}

impl<'a> AiIteratorAdapter<'a, Mesh<'a>> for Mesh<'a> {
    type Inner = *const ffi::AiMesh;

    #[inline(always)]
    fn from(inner: &'a *const ffi::AiMesh) -> Mesh<'a> {
        Mesh { raw: unsafe { inner.as_ref().expect("Mesh pointer provided by Assimp was NULL") } }
    }
}

macro_rules! impl_mesh_vector_field {
    ($field:ident) => {
        pub fn $field(&self) -> Option<&'a [AiVector3D]> {
            if self.raw.num_vertices == 0 || self.raw.$field.is_null() {
                None
            } else {
                Some(unsafe { slice::from_raw_parts(self.raw.$field, self.raw.num_vertices as usize) })
            }
        }
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

    /// Returns an iterator to all the faces in the mesh
    pub fn faces(&self) -> Option<AiIterator<'a, Face<'a>>> {
        if self.raw.num_faces == 0 || self.raw.faces.is_null() {
            None
        } else {
            Some(AiIterator::from(unsafe {
                slice::from_raw_parts(self.raw.faces, self.raw.num_faces as usize).iter()
            }))
        }
    }

    impl_mesh_vector_field!(vertices);
    impl_mesh_vector_field!(normals);
    impl_mesh_vector_field!(tangents);
    impl_mesh_vector_field!(bitangents);

    /// Gets the number of active UV(W) coordinate systems for the mesh. Meshes can have more than one.
    pub fn uv_channels(&self) -> usize {
        self.raw.num_uvs.iter().filter(|dim| **dim > 0).count()
    }

    /// Gets a specific UV(W) channel and the number of dimensions it contains.
    pub fn uv_channel(&self, index: usize) -> Option<(u32, &'a [AiVector3D])> {
        if index < ffi::MAX_NUMBER_OF_TEXTURECOORDS as usize {
            if self.raw.num_uvs[index] == 0 || self.raw.texcoords[index].is_null() {
                None
            } else {
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
    pub fn count_indices(&self) -> Option<u64> {
        if let Some(faces) = self.faces() {
            let mut num = 0;

            for ref face in faces {
                num += face.indices().len() as u64;
            }

            Some(num)
        } else {
            None
        }
    }

    /// Accumulates the indices for every face in the mesh. This is NOT zero-cost
    pub fn indices(&self) -> Option<Vec<c_uint>> {
        if let Some(faces) = self.faces() {
            let mut indices = Vec::new();

            for ref face in faces {
                indices.extend_from_slice(&face.indices());
            }

            Some(indices)
        } else {
            None
        }
    }

    /// Get the index of the material for this mesh
    #[inline(always)]
    pub fn material_index(&self) -> u32 { self.raw.material_index as u32 }

    /// Get the name of the mesh
    pub fn name(&self) -> String {
        return self.raw.name.clone().into()
    }
}
