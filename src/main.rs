use anyhow::Context;
use bevy_transform::components::Transform;
use cornell::*;
use embree4_rs::*;
use geometry::*;
use bevy_math::*;
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
    add_gltf(&mut geom, &device, &mut scene, &gltf_doc, &gltf_buff, &transform, MIRROR_MATERIAL)?;

    let mut commited_scene = scene.commit()?;

    camera.render(
        &mut image,
        &mut commited_scene,
        &geom,
        &lights,
        rays_per_pixel,
    );

    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
