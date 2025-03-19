use anyhow::Context;
use bevy_math::*;
use bevy_transform::components::Transform;
use cornell::*;
use embree4_rs::*;
use geometry::*;
use image::buffer::ConvertBuffer;
use image::{Rgb32FImage, RgbImage};
mod common;
// use std::env;
mod cornell;

mod camera;
use camera::*;

mod consts;
mod geometry;
use consts::*;

// tonemapping todo grokado, queria usar o TonyMcMapFace mas nao faco a minima por onde comecar
pub fn tonemap(image: &mut Rgb32FImage) {
    for y in 0..Consts::H {
        for x in 0..Consts::W {
            let pixel = image.get_pixel_mut(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            // Step 1: Compute luminance
            let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

            // Step 2: Compress luminance
            let compressed_luminance = luminance / (luminance + 1.0);

            // Step 3: Helmholtz-Kohlrausch effect (simplified)
            let saturation = if luminance > 0.0 {
                let max_channel = r.max(g).max(b);
                let min_channel = r.min(g).min(b);
                (max_channel - min_channel) / max_channel
            } else {
                0.0
            };
            let hk_boost = 1.0 + 0.2 * saturation;
            let adjusted_luminance = (compressed_luminance * hk_boost).clamp(0.0, 1.0);

            // Step 4: Scale colors to preserve ratios
            let scale = if luminance > 0.0 {
                adjusted_luminance / luminance
            } else {
                1.0
            };

            // Update the pixel
            pixel[0] = (r * scale).clamp(0.0, 1.0);
            pixel[1] = (g * scale).clamp(0.0, 1.0);
            pixel[2] = (b * scale).clamp(0.0, 1.0);
        }
    }
}

fn main() -> anyhow::Result<()> {
    // let args: Vec<String> = env::args().collect();

    let camera = Camera::new(
        Consts::CAM_POS,
        Consts::CAM_LOOKAT,
        Vec3::Y,
        Consts::W,
        Consts::H,
        Consts::CAM_FOV,
    );
    let mut image = Rgb32FImage::new(Consts::W, Consts::H);

    let device = Device::try_new(None)?;
    let mut scene = Scene::try_new(
        &device,
        SceneOptions {
            build_quality: embree4_sys::RTCBuildQuality::HIGH,
            flags: embree4_sys::RTCSceneFlags::ROBUST,
        },
    )?;

    let mut store = Storage::new();
    store.load_textures_batch(vec![
        "assets/textures/skybox/front.jpg",
        "assets/textures/skybox/back.jpg",
        "assets/textures/skybox/right.jpg",
        "assets/textures/skybox/left.jpg",
        "assets/textures/skybox/top.jpg",
        "assets/textures/skybox/bottom.jpg",
    ])?;
    cornell_box(&mut store, &device, &mut scene)?;

    let (gltf_doc, gltf_buff, _) = gltf::import("assets/magujo/suzanne.glb")?;
    let transform = Transform {
        translation: Vec3::new(450.0, 50.0, 150.0),
        rotation: Quat::from_rotation_y(220.0_f32.to_radians()),
        scale: Vec3::splat(50.0),
    };
    add_gltf(
        &mut store,
        &device,
        &mut scene,
        &gltf_doc,
        &gltf_buff,
        &transform,
        Material::GLASS_MATERIAL,
    )?;

    let (gltf_doc, gltf_buff, _) = gltf::import("assets/cube.glb")?;
    let transform = Transform {
        translation: Vec3::new(350.0, 50.0, 75.0),
        scale: Vec3::splat(50.0),
        ..Transform::default()
    };
    add_gltf(
        &mut store,
        &device,
        &mut scene,
        &gltf_doc,
        &gltf_buff,
        &transform,
        Material::EMISSIVE_MATERIAL,
    )?;

    add_skybox(&mut store, &device, &mut scene)?;

    let mut commited_scene = scene.commit()?;
    camera.render(&mut image, &mut commited_scene, &store);

    tonemap(&mut image);
    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
