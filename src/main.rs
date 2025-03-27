use anyhow::Context;
use glam::{Quat, Vec3};
use image::RgbImage;
use image::buffer::ConvertBuffer;
use std::io::*;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

mod camera;
mod color;
mod common;
mod configs;
mod cornell;
mod geometry;
mod raytracer;
mod renderer;
mod tonemap;

use configs::*;

fn main() -> anyhow::Result<()> {
    let instant = std::time::Instant::now();
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
    let transform = glam::Mat4::from_scale_rotation_translation(
        Vec3::splat(50.0),
        Quat::from_rotation_y(220.0_f32.to_radians()),
        Vec3::new(450.0, 50.0, 150.0),
    );
    cornell::add_gltf(
        &mut scene,
        &gltf_doc,
        &gltf_buff,
        transform,
        &geometry::Material::MIRROR_MATERIAL,
    )?;

    cornell::add_skybox(&mut scene)?;

    let scene = scene
        .build_scene(&mut raytracer_builder)
        .context("Error building scene")?;

    // configs
    let renderconfig = RenderConfig::SLOWEST_RAND;
    let camconfig = CamConfig::SLOW;
    let number_of_pixels = (camconfig.w * camconfig.h) as usize;

    let renderer = renderer::Renderer::new(scene, renderconfig);
    let camera = camera::Camera::new(camconfig);

    println!("Builing scene took: {:?}", instant.elapsed());
    let instant = std::time::Instant::now();

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

            print!("Progress: {:.2}%\r", progress);
            std::io::stdout().flush().unwrap();
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
