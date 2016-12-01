#![allow(bad_style)]

use libc::{c_char, c_uchar, c_float, c_int};
use libc::{c_uint, c_void, size_t, c_double};

use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::ffi::CStr;
use std::borrow::Cow;
use std::slice;

mod link;

#[cfg(not(feature = "double_precision"))]
pub mod types {
    //! This mod defines feature dependent types used by Assimp
    use libc::{c_float, c_int, c_uint};

    /// `AiReal` can either be a single or double precision floating point type depending on feature flags
    pub type AiReal = c_float;
    /// `AiInt` can either be a WORD or DWORD integer type depending on feature flags
    pub type AiInt = c_int;
    /// `AiUInt` can either be a WORD or DWORD Unsigned integer type depending on feature flags
    pub type AIUInt = c_uint;
}

#[cfg(feature = "double_precision")]
pub mod types {
    //! This mod defines feature dependent types used by Assimp
    use libc::{c_double, c_longlong, c_ulonglong};

    /// `AiReal` can either be a single or double precision floating point type depending on feature flags
    pub type AiReal = c_double;
    /// `AiInt` can either be a WORD or DWORD integer type depending on feature flags
    pub type AiInt = c_longlong;
    /// `AiUInt` can either be a WORD or DWORD Unsigned integer type depending on feature flags
    pub type AiUInt = c_ulonglong;
}

pub use self::types::*;

pub const FALSE: c_int = 0;
pub const TRUE: c_int = 1;

pub const AI_METADATA_TYPE_BOOL: c_int = 0;
pub const AI_METADATA_TYPE_INT32: c_int = 1;
pub const AI_METADATA_TYPE_UINT64: c_int = 2;
pub const AI_METADATA_TYPE_FLOAT: c_int = 3;
pub const AI_METADATA_TYPE_DOUBLE: c_int = 4;
pub const AI_METADATA_TYPE_AISTRING: c_int = 5;
pub const AI_METADATA_TYPE_AIVECTOR3D: c_int = 6;

pub const SCENE_FLAG_INCOMPLETE: c_uint = 0x1;
pub const SCENE_FLAG_VALIDATED: c_uint = 0x2;
pub const SCENE_FLAG_VALIDATION_WARNING: c_uint = 0x4;
pub const SCENE_FLAG_NON_VERBOSE_FORMAT: c_uint = 0x8;
pub const SCENE_FLAG_TERRAIN: c_uint = 0x10;
pub const SCENE_FLAG_ALLOW_SHARED: c_uint = 0x20;

pub const MAX_FACE_INDICES: c_uint = 0x7fff;
pub const MAX_BONE_WEIGHTS: c_uint = 0x7fffffff;
pub const MAX_VERTICES: c_uint = 0x7fffffff;
pub const MAX_FACES: c_uint = 0x7fffffff;
pub const MAX_NUMBER_OF_COLOR_SETS: c_uint = 0x8;
pub const MAX_NUMBER_OF_TEXTURECOORDS: c_uint = 0x8;

pub const PRIMITIVE_TYPE_POINT: c_uint = 0x1;
pub const PRIMITIVE_TYPE_LINE: c_uint = 0x2;
pub const PRIMITIVE_TYPE_TRIANGLE: c_uint = 0x4;
pub const PRIMITIVE_TYPE_POLYGON: c_uint = 0x8;

pub const MAXLEN: size_t = 1024;

pub const ARCH_FORMAT_HINT_LENGTH: usize = 9;

pub const TEXTURE_OP_MULTIPLY: c_int = 0x0;
pub const TEXTURE_OP_ADD: c_int = 0x1;
pub const TEXTURE_OP_SUBTRACT: c_int = 0x2;
pub const TEXTURE_OP_DIVIDE: c_int = 0x3;
pub const TEXTURE_OP_SMOOTH_ADD: c_int = 0x4;
pub const TEXTURE_OP_SIGNED_ADD: c_int = 0x5;

pub const TEXTURE_MAP_MODE_WRAP: c_int = 0x0;
pub const TEXTURE_MAP_MODE_CLAMP: c_int = 0x1;
pub const TEXTURE_MAP_MODE_MIRROR: c_int = 0x2;
pub const TEXTURE_MAP_MODE_DECAL: c_int = 0x3;

pub const TEXTURE_MAPPING_UV: c_int = 0x0;
pub const TEXTURE_MAPPING_SPHERE: c_int = 0x1;
pub const TEXTURE_MAPPING_CYLINDER: c_int = 0x2;
pub const TEXTURE_MAPPING_BOX: c_int = 0x3;
pub const TEXTURE_MAPPING_PLANE: c_int = 0x4;
pub const TEXTURE_MAPPING_OTHER: c_int = 0x5;

pub const TEXTURE_TYPE_NONE: c_int = 0x0;
pub const TEXTURE_TYPE_DIFFUSE: c_int = 0x1;
pub const TEXTURE_TYPE_SPECULAR: c_int = 0x2;
pub const TEXTURE_TYPE_AMBIENT: c_int = 0x3;
pub const TEXTURE_TYPE_EMISSIVE: c_int = 0x4;
pub const TEXTURE_TYPE_HEIGHT: c_int = 0x5;
pub const TEXTURE_TYPE_NORMALS: c_int = 0x6;
pub const TEXTURE_TYPE_SHININESS: c_int = 0x7;
pub const TEXTURE_TYPE_OPACITY: c_int = 0x8;
pub const TEXTURE_TYPE_DISPLACEMENT: c_int = 0x9;
pub const TEXTURE_TYPE_LIGHTMAP: c_int = 0xA;
pub const TEXTURE_TYPE_REFLECTION: c_int = 0xB;
pub const TEXTURE_TYPE_UNKNOWN: c_int = 0xC;

pub const SHADING_MODE_FLAT: c_int = 0x1;
pub const SHADING_MODE_GOURAUD: c_int = 0x2;
pub const SHADING_MODE_PHONG: c_int = 0x3;
pub const SHADING_MODE_BLINN: c_int = 0x4;
pub const SHADING_MODE_TOON: c_int = 0x5;
pub const SHADING_MODE_ORENNAYAR: c_int = 0x6;
pub const SHADING_MODE_MINNAERT: c_int = 0x7;
pub const SHADING_MODE_COOKTORRANCE: c_int = 0x8;
pub const SHADING_MODE_NOSHADING: c_int = 0x9;
pub const SHADING_MODE_FRESNEL: c_int = 0xA;

pub const TEXTURE_FLAG_INVERT: c_int = 0x1;
pub const TEXTURE_FLAG_USE_ALPHA: c_int = 0x2;
pub const TEXTURE_FLAG_IGNORE_ALPHA: c_int = 0x4;

pub const BLEND_MODE_DEFAULT: c_int = 0x0;
pub const BLEND_MODE_ADDITIVE: c_int = 0x1;

pub const PROPERTY_TYPE_FLOAT: c_int = 0x1;
pub const PROPERTY_TYPE_DOUBLE: c_int = 0x2;
pub const PROPERTY_TYPE_STRING: c_int = 0x3;
pub const PROPERTY_TYPE_INTEGER: c_int = 0x4;
pub const PROPERTY_TYPE_BUFFER: c_int = 0x5;

pub const LIGHT_SOURCE_UNDEFINED: c_uint = 0x0;
pub const LIGHT_SOURCE_DIRECTIONAL: c_uint = 0x1;
pub const LIGHT_SOURCE_POINT: c_uint = 0x2;
pub const LIGHT_SOURCE_SPOT: c_uint = 0x3;
pub const LIGHT_SOURCE_AMBIENT: c_uint = 0x4;
pub const LIGHT_SOURCE_AREA: c_uint = 0x5;

pub const IMPORTER_FLAG_SUPPORT_TEXT: c_int = 0x1;
pub const IMPORTER_FLAG_SUPPORT_BINARY: c_int = 0x2;
pub const IMPORTER_FLAG_SUPPORT_COMPRESSED: c_int = 0x4;
pub const IMPORTER_FLAG_SUPPORT_LIMITED: c_int = 0x8;
pub const IMPORTER_FLAG_EXPERIMENTAL: c_int = 0x10;

pub const POSTPROCESS_CALC_TANGENT_SPACE: c_uint = 0x1;
pub const POSTPROCESS_JOIN_IDENTICAL_VERTICES: c_uint = 0x2;
pub const POSTPROCESS_MAKE_LEFT_HANDED: c_uint = 0x4;
pub const POSTPROCESS_TRIANGULATE: c_uint = 0x8;
pub const POSTPROCESS_REMOVE_COMPONENT: c_uint = 0x10;
pub const POSTPROCESS_GEN_NORMALS: c_uint = 0x20;
pub const POSTPROCESS_GEN_SMOOTH_NORMALS: c_uint = 0x40;
pub const POSTPROCESS_SPLIT_LARGE_MESHES: c_uint = 0x80;
pub const POSTPROCESS_TRANSFORM_VERTICES: c_uint = 0x100;
pub const POSTPROCESS_LIMIT_BONE_WEIGHTS: c_uint = 0x200;
pub const POSTPROCESS_VALIDATE_DATA_STRUCTURE: c_uint = 0x400;
pub const POSTPROCESS_IMPROVE_CACHE_LOCALITY: c_uint = 0x800;
pub const POSTPROCESS_REMOVE_REDUNDANT_MATERIALS: c_uint = 0x1000;
pub const POSTPROCESS_FIX_INFACING_NORMALS: c_uint = 0x2000;
pub const POSTPROCESS_SORT_BY_PTYPE: c_uint = 0x8000;
pub const POSTPROCESS_FIND_DEGENERATES: c_uint = 0x10000;
pub const POSTPROCESS_FIND_INVALID_DATA: c_uint = 0x20000;
pub const POSTPROCESS_GEN_UV_COORDS: c_uint = 0x40000;
pub const POSTPROCESS_TRANSFORM_UV_COORDS: c_uint = 0x80000;
pub const POSTPROCESS_FIND_INSTANCES: c_uint = 0x100000;
pub const POSTPROCESS_OPTIMIZE_MESHES: c_uint = 0x200000;
pub const POSTPROCESS_OPTIMIZE_GRAPH: c_uint = 0x400000;
pub const POSTPROCESS_FLIP_UVS: c_uint = 0x800000;
pub const POSTPROCESS_FLIP_WINDING_ORDER: c_uint = 0x1000000;
pub const POSTPROCESS_SPLIT_BY_BONE_COUNT: c_uint = 0x2000000;
pub const POSTPROCESS_DEBONE: c_uint = 0x4000000;

pub const ANIM_BEHAVIOR_DEFAULT: c_uint = 0x0;
pub const ANIM_BEHAVIOR_CONSTANT: c_uint = 0x1;
pub const ANIM_BEHAVIOR_LINEAR: c_uint = 0x2;
pub const ANIM_BEHAVIOR_REPEAT: c_uint = 0x3;

#[repr(C)]
pub struct AiString {
    pub length: size_t,
    pub data: [c_uchar; MAXLEN as usize],
}

impl AiString {
    pub fn to_string_lossy(&self) -> Cow<str> {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(
                slice::from_raw_parts(self.data.as_ptr(), self.length as usize + 1)
            )
        }.to_string_lossy()
    }
}

impl From<AiString> for String {
    fn from(s: AiString) -> String {
        String::from_utf8_lossy(&s.data[0..s.length]).into()
    }
}

impl Clone for AiString {
    fn clone(&self) -> AiString { AiString { ..*self } }
}

impl Display for AiString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl Debug for AiString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "AiString \"{}\"", self)
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct AiVector3D {
    pub x: AiReal,
    pub y: AiReal,
    pub z: AiReal,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct AiVector2D {
    pub x: AiReal,
    pub y: AiReal,
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct AiMatrix3x3 {
    pub a1: AiReal,
    pub a2: AiReal,
    pub a3: AiReal,
    pub b1: AiReal,
    pub b2: AiReal,
    pub b3: AiReal,
    pub c1: AiReal,
    pub c2: AiReal,
    pub c3: AiReal,
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct AiMatrix4x4 {
    pub a1: AiReal,
    pub a2: AiReal,
    pub a3: AiReal,
    pub a4: AiReal,
    pub b1: AiReal,
    pub b2: AiReal,
    pub b3: AiReal,
    pub b4: AiReal,
    pub c1: AiReal,
    pub c2: AiReal,
    pub c3: AiReal,
    pub c4: AiReal,
    pub d1: AiReal,
    pub d2: AiReal,
    pub d3: AiReal,
    pub d4: AiReal,
}

#[repr(C)] //Not packed?
#[derive(Copy, Clone, Debug)]
pub struct AiQuaternion {
    pub w: AiReal,
    pub x: AiReal,
    pub y: AiReal,
    pub z: AiReal,
}


#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct AiUVTransform {
    pub translation: AiVector2D,
    pub scaling: AiVector2D,
    pub rotation: AiReal,
}

#[repr(C)]
pub struct AiMetadata {
    pub num_properties: c_uint,
    pub keys: *const AiString,
    pub values: *const AiMetadataEntry,
}

#[repr(C)]
pub struct AiMetadataEntry {
    pub kind: c_int,
    pub data: *const c_void,
}

#[repr(C)]
pub struct AiVectorKey {
    pub time: c_double,
    pub value: AiVector3D,
}

#[repr(C)]
pub struct AiQuatKey {
    pub time: c_double,
    pub value: AiQuaternion,
}

#[repr(C)]
pub struct AiMeshKey {
    pub time: c_double,
    pub value: c_uint,
}

#[repr(C)]
pub struct AiNodeAnim {
    pub name: AiString,

    pub num_position_keys: c_uint,
    pub position_keys: *const AiVectorKey,

    pub num_rotation_keys: c_uint,
    pub rotation_keys: *const AiQuatKey,

    pub num_scaling_keys: c_uint,
    pub scaling_keys: *const AiVectorKey,

    pub pre_state: c_uint,
    pub post_state: c_uint,
}

#[repr(C)]
pub struct AiMeshAnim {
    pub name: AiString,
    pub num_keys: c_uint,
    pub keys: AiMeshKey,
}

#[repr(C)]
pub struct AiAnimation {
    pub name: AiString,

    pub duration: c_double,
    pub ticks_per_second: c_double,

    pub num_channels: c_uint,
    pub channels: *const *const AiNodeAnim,

    pub num_mesh_channels: c_uint,
    pub mesh_channels: *const *const AiMeshAnim,
}

#[repr(C)]
#[derive(Debug)]
pub struct AiCamera {
    pub name: AiString,
    pub position: AiVector3D,
    pub up: AiVector3D,
    pub look_at: AiVector3D,
    pub hfov: c_float,
    pub znear: c_float,
    pub zfar: c_float,
    pub aspect: c_float,
}

#[repr(C)]
#[derive(Debug)]
pub struct AiLight {
    pub name: AiString,
    pub kind: c_int,
    pub position: AiVector3D,
    pub direction: AiVector3D,
    pub up: AiVector3D,
    pub attenuation_constant: c_float,
    pub attenuation_linear: c_float,
    pub attenuation_quadratic: c_float,
    pub diffuse_color: AiColor3D,
    pub specular_color: AiColor3D,
    pub ambient_color: AiColor3D,
    pub inner_angle: c_float,
    pub outer_angle: c_float,
    pub size: AiVector2D
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct AiColor4D {
    pub r: AiReal,
    pub g: AiReal,
    pub b: AiReal,
    pub a: AiReal,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct AiColor3D {
    pub r: AiReal,
    pub g: AiReal,
    pub b: AiReal,
}

#[repr(C)]
pub struct AiMaterialProperty {
    pub key: AiString,
    pub semantic: c_uint,
    pub index: c_uint,
    pub data_length: c_uint,
    pub property_type: c_int,
    pub data: *const c_char,
}

#[repr(C)]
pub struct AiMaterial {
    pub properties: *const *const AiMaterialProperty,
    pub num_properties: c_uint,
    pub num_allocated: c_uint,
}

#[repr(C)]
pub struct AiFace {
    pub num_indices: c_uint,
    pub indices: *const c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AiVertexWeight {
    pub vertex_id: c_uint,
    pub weight: c_float,
}

#[repr(C)]
pub struct AiBone {
    pub name: AiString,
    pub num_weights: c_uint,
    pub weights: *const AiVertexWeight,
    pub offset_matrix: AiMatrix4x4,
}

#[repr(C)]
pub struct AiAnimMesh {
    pub vertices: *const AiVector3D,
    pub normals: *const AiVector3D,
    pub tangents: *const AiVector3D,
    pub bitangents: *const AiVector3D,
    pub colors: [*const AiColor4D; MAX_NUMBER_OF_COLOR_SETS as usize],
    pub texcoords: [*const AiVector3D; MAX_NUMBER_OF_TEXTURECOORDS as usize],
    pub num_vertices: c_uint,
}

#[repr(C)]
pub struct AiMesh {
    pub primitive_type: c_uint,
    pub num_vertices: c_uint,
    pub num_faces: c_uint,
    pub vertices: *const AiVector3D,
    pub normals: *const AiVector3D,
    pub tangents: *const AiVector3D,
    pub bitangents: *const AiVector3D,
    pub colors: [*const AiColor4D; MAX_NUMBER_OF_COLOR_SETS as usize],
    pub texcoords: [*const AiVector3D; MAX_NUMBER_OF_TEXTURECOORDS as usize],
    pub num_uvs: [c_uint; MAX_NUMBER_OF_TEXTURECOORDS as usize],
    pub faces: *const AiFace,
    pub num_bones: c_uint,
    pub bones: *const *const AiBone,
    pub material_index: c_uint,
    pub name: AiString,
    //Not in use yet by Assimp
    num_anim_meshes: c_uint,
    anim_meshes: *const *const AiAnimMesh,
}

#[repr(C)]
pub struct AiNode {
    pub name: AiString,
    pub transformation: AiMatrix4x4,
    pub parent: *const AiNode,
    pub num_children: c_uint,
    pub children: *const *const AiNode,
    pub num_meshes: c_uint,
    pub meshes: *const c_uint,
    pub metadata: *const AiMetadata,
}

#[repr(C)]
pub struct AiTexture {
    pub width: c_uint,
    pub height: c_uint,
    pub arch_format_hint: [c_char; ARCH_FORMAT_HINT_LENGTH],
    pub data: *const AiTexel,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct AiTexel {
    pub b: c_uint,
    pub g: c_uint,
    pub r: c_uint,
    pub a: c_uint,
}

#[repr(C)]
pub struct AiScene {
    pub flags: c_uint,

    pub root_node: *const AiNode,

    pub num_meshes: c_uint,
    pub meshes: *const *const AiMesh,

    pub num_materials: c_uint,
    pub materials: *const *const AiMaterial,

    pub num_animations: c_uint,
    pub animations: *const *const AiAnimation,

    pub num_textures: c_uint,
    pub textures: *const *const AiTexture,

    pub num_lights: c_uint,
    pub lights: *const *const AiLight,

    pub num_cameras: c_uint,
    pub cameras: *const *const AiCamera,
    //Unused by us
    _private: *const c_void,
}

extern "C" {
    pub fn aiApplyPostProcessing(scene: *const AiScene, flags: c_uint) -> *const AiScene;

    pub fn aiDetachAllLogStreams();
    pub fn aiEnableVerboseLogging(enable: c_int);

    pub fn aiGetErrorString() -> *const c_char;

    pub fn aiGetExtensionList(out: *mut AiString);

    pub fn aiImportFile(path: *const c_char, flags: c_uint) -> *const AiScene;

    pub fn aiIsExtensionSupported(extension: *const c_char) -> c_int;

    pub fn aiReleaseImport(scene: *const AiScene);
}