extern crate assimp;

use assimp::*;

fn main() {
    let effects = postprocess::PostprocessEffectBuilder::target_realtime_max_quality()
        .gen_normals(false)
        .gen_smooth_normals(false);

    let scene: Scene = Scene::import("./examples/twilight.obj", None).unwrap();

    println!("Scene loaded.\nShowing meshes...");

    if let Some(meshes) = scene.meshes() {
        for mesh in meshes {
            let vertices = mesh.vertices().unwrap();

            println!("Found mesh {} with {} vertices and {} indices!", mesh.name(), vertices.len(), mesh.count_indices().unwrap());
            println!("Also {} UV channels", mesh.uv_channels());
        }
    } else {
        println!("No meshes found");
    }

    if let Some(materials) = scene.materials() {
        for material in materials {
            println!("Found material!");
        }
    } else {
        println!("No materials found");
    }
}