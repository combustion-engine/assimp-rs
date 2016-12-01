//! Nalgebra compatibility module
//!
//! This module implements the `From` trait to convert between Assimp structures and Nalgebra structures.

use nalgebra::*;

use ffi;

pub use ffi::{AiVector3D, AiVector2D, AiMatrix4x4, AiMatrix3x3, AiQuaternion, AiUVTransform};

impl From<ffi::AiVector2D> for Vector2<ffi::AiReal> {
    #[inline]
    fn from(v: ffi::AiVector2D) -> Vector2<ffi::AiReal> {
        Vector2::new(v.x, v.y)
    }
}

impl From<Vector2<ffi::AiReal>> for ffi::AiVector2D {
    #[inline]
    fn from(v: Vector2<ffi::AiReal>) -> ffi::AiVector2D {
        ffi::AiVector2D { x: v.x, y: v.y }
    }
}

impl From<ffi::AiVector3D> for Vector3<ffi::AiReal> {
    #[inline]
    fn from(v: ffi::AiVector3D) -> Vector3<ffi::AiReal> {
        Vector3::new(v.x, v.y, v.z)
    }
}

impl From<Vector3<ffi::AiReal>> for ffi::AiVector3D {
    #[inline]
    fn from(v: Vector3<ffi::AiReal>) -> ffi::AiVector3D {
        ffi::AiVector3D { x: v.x, y: v.y, z: v.z }
    }
}

impl From<ffi::AiMatrix3x3> for Matrix3<ffi::AiReal> {
    fn from(v: ffi::AiMatrix3x3) -> Matrix3<ffi::AiReal> {
        Matrix3::new(v.a1, v.a2, v.a3,
                     v.b1, v.b2, v.b3,
                     v.c1, v.c2, v.c3)
    }
}

impl From<Matrix3<ffi::AiReal>> for ffi::AiMatrix3x3 {
    fn from(v: Matrix3<ffi::AiReal>) -> ffi::AiMatrix3x3 {
        ffi::AiMatrix3x3 {
            a1: v.m11, a2: v.m12, a3: v.m13,
            b1: v.m21, b2: v.m22, b3: v.m23,
            c1: v.m31, c2: v.m32, c3: v.m33,
        }
    }
}

impl From<ffi::AiMatrix4x4> for Matrix4<ffi::AiReal> {
    fn from(v: ffi::AiMatrix4x4) -> Matrix4<ffi::AiReal> {
        Matrix4::new(v.a1, v.a2, v.a3, v.a4,
                     v.b1, v.b2, v.b3, v.b4,
                     v.c1, v.c2, v.c3, v.c4,
                     v.d1, v.d2, v.d3, v.d4)
    }
}

impl From<Matrix4<ffi::AiReal>> for ffi::AiMatrix4x4 {
    fn from(v: Matrix4<ffi::AiReal>) -> ffi::AiMatrix4x4 {
        ffi::AiMatrix4x4 {
            a1: v.m11, a2: v.m12, a3: v.m13, a4: v.m14,
            b1: v.m21, b2: v.m22, b3: v.m23, b4: v.m24,
            c1: v.m31, c2: v.m32, c3: v.m33, c4: v.m34,
            d1: v.m41, d2: v.m42, d3: v.m43, d4: v.m44,
        }
    }
}

impl ffi::AiUVTransform {
    /// Get the translation and rotation parts of a UV Transform as an Nalgebra `Isometry2`
    pub fn isometry(&self) -> Isometry2<ffi::AiReal> {
        Isometry2::new(self.translation.clone().into(), Vector1::new(self.rotation))
    }
}

impl From<Quaternion<ffi::AiReal>> for ffi::AiQuaternion {
    #[inline]
    fn from(v: Quaternion<ffi::AiReal>) -> ffi::AiQuaternion {
        ffi::AiQuaternion { w: v.w, x: v.i, y: v.j, z: v.k }
    }
}

impl From<ffi::AiQuaternion> for Quaternion<ffi::AiReal> {
    #[inline]
    fn from(v: ffi::AiQuaternion) -> Quaternion<ffi::AiReal> {
        Quaternion { w: v.w, i: v.x, j: v.y, k: v.z }
    }
}