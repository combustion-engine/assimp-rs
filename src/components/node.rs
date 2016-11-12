use std::slice;
use std::marker::PhantomData;
use libc::c_uint;

use enum_primitive::FromPrimitive;

use ::ffi;
use ::ffi::*;

use iterator::*;

pub struct Node<'a> {
    raw: &'a ffi::AiNode
}

impl<'a> AiIteratorAdapter<'a, Node<'a>> for Node<'a> {
    type Inner = ffi::AiNode;

    #[inline(always)]
    fn from(inner: &'a ffi::AiNode) -> Node<'a> {
        Node { raw: inner }
    }
}