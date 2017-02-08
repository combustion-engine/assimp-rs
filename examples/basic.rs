extern crate assimp;

use std::iter::repeat;
use std::fs::File;

use assimp::*;

fn format_node<'a>(scene: &'a Scene<'a>, node: &'a Node<'a>, depth: usize) {
    let space = repeat("  ").take(depth).collect::<String>();

    let info = format!("{}", node.name());

    if let Some(children) = node.children() {
        println!("{}{}---v", space, info);

        for child in children {
            format_node(scene, &child, depth + 1);
        }
    } else {
        println!("{}{}", space, info);
    }
}

fn main() {
    let effects = postprocess::PostprocessEffectBuilder::target_realtime_max_quality()
        .gen_normals(false)
        .transform_vertices(true)
        .gen_smooth_normals(false);

    let mut io = io::CustomIO::callback(|path| {
        File::open(path)
    });

    let scene: Scene = Scene::import_from("./examples/sphere.dae", None, &mut io).unwrap();

    println!("Scene loaded.");

    format_node(&scene, &scene.root(), 0);

    println!("Showing meshes...");

    if let Some(_) = scene.mesh(0) {
        println!("Has zero-index mesh");
    }

    if let Some(meshes) = scene.meshes() {
        for mesh in meshes {
            let vertices = mesh.vertices().unwrap();

            println!("Found mesh `{}` with {} vertices and {} indices!", mesh.name(), vertices.len(), mesh.count_indices().unwrap());
            println!("Also {} UV channels", mesh.uv_channels());
        }
    } else {
        println!("No meshes found");
    }

    if let Some(lights) = scene.lights() {
        for light in lights {
            println!("Found {:?}", light);
        }
    }

    if let Some(materials) = scene.materials() {
        for _ in materials {
            println!("Found material!");
        }
    } else {
        println!("No materials found");
    }
}