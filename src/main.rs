use crate::consts::Consts;
use anyhow::Context;
use bevy_math::{Quat, Vec3};
use bevy_transform::components::Transform;
use image::RgbImage;
use image::buffer::ConvertBuffer;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

mod common;
mod cornell;

mod camera;

mod consts;
mod geometry;
mod raytracer;
mod renderer;
mod tonemap;

fn main() -> anyhow::Result<()> {
    let mut instant = std::time::Instant::now();
    let device = embree4_rs::Device::try_new(None)?;
    let mut raytracer_builder = raytracer::embree::EmbreeRayTracerBuilder::new(&device);

    let mut scene = geometry::Scene::new()?;
    scene.add_textures_batch_from_files(&vec![
        "assets/textures/skybox/front.jpg",
        "assets/textures/skybox/back.jpg",
        "assets/textures/skybox/right.jpg",
        "assets/textures/skybox/left.jpg",
        "assets/textures/skybox/top.jpg",
        "assets/textures/skybox/bottom.jpg",
    ])?;
    cornell::cornell_box(&mut scene)?;

    let (gltf_doc, gltf_buff, _) = gltf::import("assets/magujo/suzanne.glb")?;
    let transform = Transform {
        translation: Vec3::new(450.0, 50.0, 150.0),
        rotation: Quat::from_rotation_y(220.0_f32.to_radians()),
        scale: Vec3::splat(50.0),
    };
    cornell::add_gltf(
        &mut scene,
        &gltf_doc,
        &gltf_buff,
        transform.compute_matrix(),
        &geometry::Material::MIRROR_MATERIAL,
    )?;

    cornell::add_skybox(&mut scene)?;

    let scene = scene
        .build_scene(&mut raytracer_builder)
        .context("Error building scene")?;

    let renderer = renderer::Renderer::new(scene);
    let camera = camera::Camera::new(
        Consts::CAM_POS,
        Consts::CAM_LOOKAT,
        Vec3::Y,
        Consts::W,
        Consts::H,
        Consts::CAM_FOV,
    );

    println!("Builing scene took: {:?}", instant.elapsed());
    let instant = std::time::Instant::now();

    let number_of_pixels = (Consts::W * Consts::H) as usize;
    let rendered_pixels = Arc::new(AtomicUsize::new(0));

    let progress_rendered_pixels = Arc::clone(&rendered_pixels);
    std::thread::spawn(move || {
        loop {
            let progress = (progress_rendered_pixels.load(std::sync::atomic::Ordering::Relaxed)
                as f32
                / number_of_pixels as f32)
                * 100.0;

            if progress >= 100.0 {
                break;
            }

            println!("Progress: {:.2}%", progress);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let mut image = renderer.render_par_with_progress(&camera, rendered_pixels);

    tonemap::tonemap(&mut image);

    println!("Render complete in: {:?}", instant.elapsed());

    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
