//! Contains all formats and their file extensions supported by Assimp
//!
//! See http://www.assimp.org/main_features_formats.html

use phf;

/// Contains all Assimp file formats and their extensions
///
/// Some file formats have multiple extensions
pub static IMPORT_FORMATS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    // COMMON INTERCHANGE FORMATS
    "Autodesk"                                       => &["fbx"]            as &'static [&'static str],
    "Collada"                                        => &["dae"]            as &'static [&'static str],
    "glTF"                                           => &["gltf", "glb"]    as &'static [&'static str],
    "Blender 3D"                                     => &["blend"]          as &'static [&'static str],
    "3ds Max 3DS"                                    => &["3ds"]            as &'static [&'static str],
    "3ds Max ASE"                                    => &["ase"]            as &'static [&'static str],
    "Wavefront Object"                               => &["obj"]            as &'static [&'static str],
    "Industry Foundation Classes (IFC/Step)"         => &["ifc"]            as &'static [&'static str],
    "XGL"                                            => &["xgl", "zgl"]     as &'static [&'static str],
    "Stanford Polygon Library"                       => &["ply"]            as &'static [&'static str],
    "AutoCAD DXF"                                    => &["dxf"]            as &'static [&'static str],
    "LightWave"                                      => &["lwo"]            as &'static [&'static str],
    "LightWave Scene"                                => &["lws"]            as &'static [&'static str],
    "Modo"                                           => &["lxo"]            as &'static [&'static str],
    "Stereolithography"                              => &["stl"]            as &'static [&'static str],
    "DirectX X"                                      => &["x"]              as &'static [&'static str],
    "AC3D"                                           => &["ac"]             as &'static [&'static str],
    "Milkshape 3D"                                   => &["ms3d"]           as &'static [&'static str],
    "TrueSpace"                                      => &["cob", "scn"]     as &'static [&'static str],
    // MOTION CAPTURE FORMATS
    "Biovision BVH"                                  => &["bvh"]            as &'static [&'static str],
    "CharacterStudio Motion"                         => &["csm"]            as &'static [&'static str],
    // GRAPHICS ENGINE FORMATS
    "Ogre XML"                                       => &["xml"]            as &'static [&'static str],
    "Irrlicht Mesh"                                  => &["irrmesh"]        as &'static [&'static str],
    "Irrlicht Scene"                                 => &["irr"]            as &'static [&'static str],
    // GAME FILE FORMATS
    "Quake I"                                        => &["mdl"]            as &'static [&'static str],
    "Quake II"                                       => &["md2"]            as &'static [&'static str],
    "Quake III Mesh"                                 => &["md3"]            as &'static [&'static str],
    "Quake III Map/BSP"                              => &["pk3"]            as &'static [&'static str],
    "Return to Castle Wolfenstein"                   => &["mdc"]            as &'static [&'static str],
    "Doom 3"                                         => &["md5mesh",
                                                          "md5anim"]        as &'static [&'static str],
    "Valve Model"                                    => &["smd", "vta"]     as &'static [&'static str],
    "Open Game Engine Exchange"                      => &["ogex"]           as &'static [&'static str],
    "Unreal"                                         => &["3d"]             as &'static [&'static str],
    // OTHER FILE FORMATS
    "BlitzBasic 3D"                                  => &["b3d"]            as &'static [&'static str],
    "Quick3D"                                        => &["q3d", "q3s"]     as &'static [&'static str],
    "Neutral File Format"                            => &["nff"]            as &'static [&'static str],
    "Sense8 WorldToolKit"                            => &["nff"]            as &'static [&'static str],
    "Object File Format"                             => &["off"]            as &'static [&'static str],
    "PovRAY Raw"                                     => &["raw"]            as &'static [&'static str],
    "Terragen Terrain"                               => &["ter"]            as &'static [&'static str],
    "3D GameStudio (3DGS)"                           => &["mdl"]            as &'static [&'static str],
    "3D GameStudio (3DGS) Terrain"                   => &["hmp"]            as &'static [&'static str],
    "Izware Nendo"                                   => &["ndo"]            as &'static [&'static str],
};

/// Contains all the extensions Assimp supports and what file format they belong to
///
/// Some extensions share multiple file formats
pub static IMPORT_EXTENSIONS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    // COMMON INTERCHANGE FORMATS
    "fbx"       => &["Autodesk"]                                            as &'static [&'static str],
    "dae"       => &["Collada"]                                             as &'static [&'static str],
    "gltf"      => &["glTF"]                                                as &'static [&'static str],
    "glb"       => &["glTF"]                                                as &'static [&'static str],
    "blend"     => &["Blender 3D"]                                          as &'static [&'static str],
    "3ds"       => &["3ds Max 3DS"]                                         as &'static [&'static str],
    "ase"       => &["3ds Max ASE"]                                         as &'static [&'static str],
    "obj"       => &["Wavefront Object"]                                    as &'static [&'static str],
    "ifc"       => &["Industry Foundation Classes (IFC/Step)"]              as &'static [&'static str],
    "xgl"       => &["XGL"]                                                 as &'static [&'static str],
    "zgl"       => &["XGL"]                                                 as &'static [&'static str],
    "ply"       => &["Stanford Polygon Library"]                            as &'static [&'static str],
    "dxf"       => &["AutoCAD DXF"]                                         as &'static [&'static str],
    "lwo"       => &["LightWave"]                                           as &'static [&'static str],
    "lws"       => &["LightWave Scene"]                                     as &'static [&'static str],
    "lxo"       => &["Modo"]                                                as &'static [&'static str],
    "stl"       => &["Stereolithography"]                                   as &'static [&'static str],
    "x"         => &["DirectX X"]                                           as &'static [&'static str],
    "ac"        => &["AC3D"]                                                as &'static [&'static str],
    "ms3d"      => &["Milkshape 3D"]                                        as &'static [&'static str],
    "cob"       => &["TrueSpace"]                                           as &'static [&'static str],
    "scn"       => &["TrueSpace"]                                           as &'static [&'static str],
    // MOTION CAPTURE FORMATS
    "bvh"       => &["Biovision BVH"]                                       as &'static [&'static str],
    "csm"       => &["CharacterStudio Motion"]                              as &'static [&'static str],
    // GRAPHICS ENGINE FORMATS
    "xml"       => &["Ogre XML"]                                            as &'static [&'static str],
    "irrmesh"   => &["Irrlicht Mesh"]                                       as &'static [&'static str],
    "irr"       => &["Irrlicht Scene"]                                      as &'static [&'static str],
    // GAME FILE FORMATS
    "mdl"       => &["Quake I", "3D GameStudio (3DGS)"]                     as &'static [&'static str],
    "md2"       => &["Quake II"]                                            as &'static [&'static str],
    "md3"       => &["Quake III Mesh"]                                      as &'static [&'static str],
    "pk3"       => &["Quake III Map/BSP"]                                   as &'static [&'static str],
    "mdc"       => &["Return to Castle Wolfenstein"]                        as &'static [&'static str],
    "md5mesh"   => &["Doom 3"]                                              as &'static [&'static str],
    "md5anim"   => &["Doom 3"]                                              as &'static [&'static str],
    "smd"       => &["Valve Model"]                                         as &'static [&'static str],
    "vta"       => &["Valve Model"]                                         as &'static [&'static str],
    "ogex"      => &["Open Game Engine Exchange"]                           as &'static [&'static str],
    "3d"        => &["Unreal"]                                              as &'static [&'static str],
    // OTHER FILE FORMATS
    "b3d"       => &["BlitzBasic 3D"]                                       as &'static [&'static str],
    "q3d"       => &["Quick3D"]                                             as &'static [&'static str],
    "q3s"       => &["Quick3D"]                                             as &'static [&'static str],
    "nff"       => &["Neutral File Format", "Sense8 WorldToolKit"]          as &'static [&'static str],
    "off"       => &["Object File Format"]                                  as &'static [&'static str],
    "raw"       => &["PovRAY Raw"]                                          as &'static [&'static str],
    "ter"       => &["Terragen Terrain"]                                    as &'static [&'static str],
    "hmp"       => &["3D GameStudio (3DGS) Terrain"]                        as &'static [&'static str],
    "ndo"       => &["Izware Nendo"]                                        as &'static [&'static str],
};

/// Specific file formats that are only partially supported by Assimp
pub static PARTIALLY_SUPPORTED_IMPORT_FORMATS: phf::Set<&'static str> = phf_set! {
    "AutoCAD DXF",
    "TrueSpace",
    "CharacterStudio Motion",
    "Irrlicht Scene",
    "Return to Castle Wolfenstein",
    "Valve Model",
    "Open Game Engine Exchange",
    "Unreal"
};

/// Specific file extensions that correlate to formats only partially supported by Assimp
pub static PARTIALLY_SUPPORTED_IMPORT_EXTENSIONS: phf::Set<&'static str> = phf_set! {
    "dxf", "cob", "scn", "csm", "irr", "mdc", "smd", "vta", "ogex", "3d"
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matching_import_formats() {
        for format in PARTIALLY_SUPPORTED_IMPORT_FORMATS.iter() {
            assert!(IMPORT_FORMATS.contains_key(format));
        }
    }

    #[test]
    fn test_matching_import_extensions() {
        for ext in PARTIALLY_SUPPORTED_IMPORT_EXTENSIONS.iter() {
            assert!(IMPORT_EXTENSIONS.contains_key(ext));
        }
    }
}