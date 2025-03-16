use anyhow::Context;
use bevy_math::*;
use bevy_transform::components::Transform;
use cornell::*;
use embree4_rs::*;
use geometry::*;
use image::buffer::ConvertBuffer;
use image::{Rgb32FImage, RgbImage};
mod common;
use std::env;
mod cornell;

mod camera;
use camera::*;

mod geometry;

const W: u32 = 640;
const H: u32 = 640;

// tonemapping todo grokado, queria usar o TonyMcMapFace mas nao faco a minima por onde comecar
pub fn tonemap(image: &mut Rgb32FImage) {
    for y in 0..H {
        for x in 0..W {
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
    let args: Vec<String> = env::args().collect();

    let mut rays_per_pixel = 5; // low on purpose for dev speed. use 20 or something
    if args.len() > 1 {
        match args[1].parse::<u32>() {
            Ok(num) => rays_per_pixel = num,
            _ => {
                panic!("Provided value {} is not a valid u32", args[1]);
            }
        }
    }

    let camera = Camera::new(
        Vec3::new(280.0, 265.0, -500.0),
        Vec3::new(280.0, 260.0, 0.0),
        Vec3::Y,
        W,
        H,
        60.0_f32.to_radians(),
    );
    let mut image = Rgb32FImage::new(W, H);

    let device = Device::try_new(None)?;
    let mut scene = Scene::try_new(
        &device,
        SceneOptions {
            build_quality: embree4_sys::RTCBuildQuality::HIGH,
            flags: embree4_sys::RTCSceneFlags::ROBUST,
        },
    )?;

    let mut geom = GeomStorage::default();
    let mut lights = LightStorage::default();
    cornell_box(&mut geom, &mut lights, &device, &mut scene)?;

    let (gltf_doc, gltf_buff, _) = gltf::import("assets/magujo/suzanne.glb")?;
    let transform = Transform {
        translation: Vec3::new(450.0, 50.0, 150.0),
        rotation: Quat::from_rotation_y(220.0_f32.to_radians()),
        scale: Vec3::splat(50.0),
    };
    add_gltf(
        &mut geom,
        &device,
        &mut scene,
        &gltf_doc,
        &gltf_buff,
        &transform,
        GLASS_MATERIAL,
    )?;
    let mut commited_scene = scene.commit()?;

    camera.render(
        &mut image,
        &mut commited_scene,
        &geom,
        &lights,
        rays_per_pixel,
    );

    tonemap(&mut image);
    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
