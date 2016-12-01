use libc::c_uint;
use ffi;

bitflags! {
    /// Postprocess Effects bitflags
    pub flags PostprocessEffect: c_uint {
        const CALC_TANGENT_SPACE = ffi::POSTPROCESS_CALC_TANGENT_SPACE,
        const JOIN_IDENTICAL_VERTICES = ffi::POSTPROCESS_JOIN_IDENTICAL_VERTICES,
        const MAKE_LEFT_HANDED = ffi::POSTPROCESS_MAKE_LEFT_HANDED,
        const TRIANGULATE = ffi::POSTPROCESS_TRIANGULATE,
        const REMOVE_COMPONENT = ffi::POSTPROCESS_REMOVE_COMPONENT,
        const GEN_NORMALS = ffi::POSTPROCESS_GEN_NORMALS,
        const GEN_SMOOTH_NORMALS = ffi::POSTPROCESS_GEN_SMOOTH_NORMALS,
        const SPLIT_LARGE_MESHES = ffi::POSTPROCESS_SPLIT_LARGE_MESHES,
        const TRANSFORM_VERTICES = ffi::POSTPROCESS_TRANSFORM_VERTICES,
        const LIMIT_BONE_WEIGHTS = ffi::POSTPROCESS_LIMIT_BONE_WEIGHTS,
        const VALIDATE_DATA_STRUCTURE = ffi::POSTPROCESS_VALIDATE_DATA_STRUCTURE,
        const IMPROVE_CACHE_LOCALITY = ffi::POSTPROCESS_IMPROVE_CACHE_LOCALITY,
        const REMOVE_REDUNDANT_MATERIALS = ffi::POSTPROCESS_REMOVE_REDUNDANT_MATERIALS,
        const FIX_INFACING_NORMALS = ffi::POSTPROCESS_FIX_INFACING_NORMALS,
        const SORT_BY_PTYPE = ffi::POSTPROCESS_SORT_BY_PTYPE,
        const FIND_DEGENERATES = ffi::POSTPROCESS_FIND_DEGENERATES,
        const FIND_INVALID_DATA = ffi::POSTPROCESS_FIND_INVALID_DATA,
        const GEN_UV_COORDS = ffi::POSTPROCESS_GEN_UV_COORDS,
        const TRANSFORM_UV_COORDS = ffi::POSTPROCESS_TRANSFORM_UV_COORDS,
        const FIND_INSTANCES = ffi::POSTPROCESS_FIND_INSTANCES,
        const OPTIMIZE_MESHES = ffi::POSTPROCESS_OPTIMIZE_MESHES,
        const OPTIMIZE_GRAPH = ffi::POSTPROCESS_OPTIMIZE_GRAPH,
        const FLIP_UVS = ffi::POSTPROCESS_FLIP_UVS,
        const FLIP_WINDING_ORDER = ffi::POSTPROCESS_FLIP_WINDING_ORDER,
        const SPLIT_BY_BONE_COUNT = ffi::POSTPROCESS_SPLIT_BY_BONE_COUNT,
        const DEBONE = ffi::POSTPROCESS_DEBONE,
    }
}

pub mod presets {
    use super::*;

    lazy_static! {
            pub static ref CONVERT_TO_LEFT_HANDED: PostprocessEffect =
                MAKE_LEFT_HANDED | FLIP_UVS | FLIP_WINDING_ORDER;

            pub static ref TARGET_REALTIME_FAST: PostprocessEffect =
                CALC_TANGENT_SPACE | JOIN_IDENTICAL_VERTICES | TRIANGULATE | SORT_BY_PTYPE;

            pub static ref TARGET_REALTIME_QUALITY: PostprocessEffect =
                *TARGET_REALTIME_FAST | IMPROVE_CACHE_LOCALITY | LIMIT_BONE_WEIGHTS | OPTIMIZE_MESHES | OPTIMIZE_GRAPH |
                REMOVE_REDUNDANT_MATERIALS | SPLIT_LARGE_MESHES | GEN_UV_COORDS | FIND_DEGENERATES | FIND_INVALID_DATA;

            pub static ref TARGET_REALTIME_MAX_QUALITY: PostprocessEffect =
                *TARGET_REALTIME_QUALITY | FIND_INSTANCES | VALIDATE_DATA_STRUCTURE;
        }
}


pub struct PostprocessEffectBuilder {
    pub effects: PostprocessEffect
}

macro_rules! impl_builder_effect {
    ($effect:ident, $name:ident) => {
        #[inline(always)]
        pub fn $name(mut self, enable: bool) -> Self {
            if enable {
                self.effects.insert($effect);
            } else {
                self.effects.remove($effect);
            }

            self
        }
    }
}

macro_rules! impl_builder_preset {
    ($preset:ident, $name:ident) => {
        /// Builder for preset of the same name
        #[inline(always)]
        pub fn $name() -> PostprocessEffectBuilder {
            PostprocessEffectBuilder {
                effects: *presets::$preset
            }
        }
    }
}

impl PostprocessEffectBuilder {
    #[inline(always)]
    pub fn new() -> PostprocessEffectBuilder {
        PostprocessEffectBuilder { effects: PostprocessEffect::empty() }
    }

    #[inline(always)]
    pub fn build(self) -> PostprocessEffect {
        self.effects
    }

    impl_builder_preset!(CONVERT_TO_LEFT_HANDED, convert_to_left_handed);
    impl_builder_preset!(TARGET_REALTIME_FAST, target_realtime_fast);
    impl_builder_preset!(TARGET_REALTIME_QUALITY, target_realtime_quality);
    impl_builder_preset!(TARGET_REALTIME_MAX_QUALITY, target_realtime_max_quality);

    impl_builder_effect!(CALC_TANGENT_SPACE, calc_tangent_space);
    impl_builder_effect!(JOIN_IDENTICAL_VERTICES, join_identical_vertices);
    impl_builder_effect!(MAKE_LEFT_HANDED, make_left_handed);
    impl_builder_effect!(TRIANGULATE, triangulate);
    impl_builder_effect!(REMOVE_COMPONENT, remove_component);
    impl_builder_effect!(GEN_NORMALS, gen_normals);
    impl_builder_effect!(GEN_SMOOTH_NORMALS, gen_smooth_normals);
    impl_builder_effect!(SPLIT_LARGE_MESHES, split_large_meshes);
    impl_builder_effect!(TRANSFORM_VERTICES, transform_vertices);
    impl_builder_effect!(LIMIT_BONE_WEIGHTS, limit_bone_weights);
    impl_builder_effect!(VALIDATE_DATA_STRUCTURE, validate_data_structure);
    impl_builder_effect!(IMPROVE_CACHE_LOCALITY, improve_cache_locality);
    impl_builder_effect!(REMOVE_REDUNDANT_MATERIALS, remove_redundant_materials);
    impl_builder_effect!(FIX_INFACING_NORMALS, fix_infacing_normals);
    impl_builder_effect!(SORT_BY_PTYPE, sort_by_ptype);
    impl_builder_effect!(FIND_DEGENERATES, find_degenerates);
    impl_builder_effect!(FIND_INVALID_DATA, find_invalid_data);
    impl_builder_effect!(GEN_UV_COORDS, gen_uv_coords);
    impl_builder_effect!(TRANSFORM_UV_COORDS, transform_uv_coords);
    impl_builder_effect!(FIND_INSTANCES, find_instances);
    impl_builder_effect!(OPTIMIZE_MESHES, optimize_meshes);
    impl_builder_effect!(OPTIMIZE_GRAPH, optimize_graph);
    impl_builder_effect!(FLIP_UVS, flip_uvs);
    impl_builder_effect!(FLIP_WINDING_ORDER, flip_winding_order);
    impl_builder_effect!(SPLIT_BY_BONE_COUNT, split_by_bone_count);
    impl_builder_effect!(DEBONE, debone);
}