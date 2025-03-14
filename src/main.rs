use anyhow::Context;
use image::buffer::ConvertBuffer;
use image::{Rgb32FImage, RgbImage};
use embree4_rs::*;
use glam::*;
use mesh::{Mesh, MeshStorage};
use bevy_color::LinearRgba;

mod common;
use common::*;

mod camera;
use camera::*;

mod mesh;

const W: u32 = 640;
const H: u32 = 640;

fn main() -> anyhow::Result<()> {

    let camera = Camera::new(Vec3::new(280.0, 265.0, -500.0), Vec3::new(280.0, 260.0, 0.0), Vec3::Y, W, H, 60.0_f32.to_radians());
    let mut image = Rgb32FImage::new(W, H);

    let device = Device::try_new(None)?;
    let mut scene = Scene::try_new(
        &device,
        SceneOptions {
            build_quality: embree4_sys::RTCBuildQuality::HIGH,
            flags: embree4_sys::RTCSceneFlags::ROBUST,
        },
    )?;

    
    
    let mut ceiling = Mesh::new();
    ceiling.material.color = LinearRgba::rgb(0.9, 0.9, 0.9);
    ceiling.verts.push((556.0, 548.8, 0.0));
    ceiling.verts.push((0.0, 548.8, 0.0));
    ceiling.verts.push((0.0, 548.8, 559.2));
    ceiling.verts.push((556.0, 548.8, 559.2));

    ceiling.indices.push((0, 1, 2));
    ceiling.indices.push((0, 2, 3));

    ceiling.verts.push((280.0, 265.0 + 20.0, -450.0));
    ceiling.verts.push((280.0 - 20.0, 265.0 - 20.0, -450.0));
    ceiling.verts.push((280.0 + 20.0, 265.0 - 20.0, -450.0));
    ceiling.indices.push((4, 5, 6));

    // mesh.verts.push((0.0, 50.0, 1.0));
    // mesh.verts.push((100.0, -50.0, 1.0));
    // mesh.verts.push((-100.0, -50.0, 1.0));
    // mesh.indices.push((0, 1, 2));

    let mut storage = MeshStorage::default();
    let _mesh_id: u32 = storage.attach(ceiling, &device, &mut scene)?;

    let mut commited_scene = scene.commit()?;

    camera.render(&mut image, &mut commited_scene, &storage, 20);

    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
