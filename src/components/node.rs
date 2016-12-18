use std::slice;
use libc::c_uint;
use std::borrow::Cow;

use ::ffi;
use ::ffi::*;

use traits::{Named, FromRaw};
use mesh::*;
use ::scene::*;

pub struct Node<'a> {
    raw: &'a ffi::AiNode
}

impl<'a> Named<'a> for Node<'a> {
    #[inline]
    fn name(&self) -> Cow<'a, str> {
        self.raw.name.to_string_lossy()
    }
}

impl<'a> FromRaw<'a, Node<'a>> for Node<'a> {
    type Raw = *const ffi::AiNode;

    #[inline(always)]
    fn from_raw(raw: &'a Self::Raw) -> Node<'a> {
        Node { raw: unsafe { raw.as_ref().expect("Node pointer provided by Assimp was NULL") } }
    }
}

impl<'a> Node<'a> {
    /// Returns the transformation matrix of this node in the scene
    #[inline(always)]
    pub fn transformation(&self) -> &AiMatrix4x4 { &self.raw.transformation }

    /// If the node has a parent, get it.
    pub fn parent(&self) -> Option<Node<'a>> {
        if self.raw.parent.is_null() { None } else {
            Some(Node { raw: unsafe { &*self.raw.parent } })
        }
    }

    impl_optional_iterator!(children, children, num_children, Node, {
        /// Returns an iterator to the children of this node
    });

    impl_optional_slice!(meshes, meshes, num_meshes, c_uint, {
        /// Returns the mesh indices for this node.
    });

    /// Maps the mesh indices of this node to the meshes in a scene.
    pub fn meshes_from_scene(&self, scene: &'a Scene<'a>) -> Option<impl Iterator<Item = Mesh<'a>> + 'a> {
        if let Some(mesh_indices) = self.meshes() {
            Some(mesh_indices
                .iter()
                .filter_map(move |index| scene.mesh(*index as usize)))
        } else {
            None
        }
    }
}