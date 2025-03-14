use anyhow::Context;
use cornell::cornell_box;
use embree4_rs::*;
use glam::*;
use image::buffer::ConvertBuffer;
use image::{Rgb32FImage, RgbImage};
use mesh::*;
mod common;
// use common::*;

mod cornell;

mod camera;
use camera::*;

mod mesh;

const W: u32 = 640;
const H: u32 = 640;

fn main() -> anyhow::Result<()> {
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

    let mut meshes = MeshStorage::default();
    let mut lights = LightStorage::default();
    cornell_box(&mut meshes, &mut lights, &device, &mut scene)?;

    let mut commited_scene = scene.commit()?;

    camera.render(&mut image, &mut commited_scene, &meshes, &lights, 20);

    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
