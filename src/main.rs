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

    let camera = Camera::new(Vec3::ZERO, Vec3::Z, Vec3::Y, W, H, 60.0_f32.to_radians());
    let mut image = Rgb32FImage::new(W, H);

    let device = Device::try_new(None)?;
    let mut scene = Scene::try_new(
        &device,
        SceneOptions {
            build_quality: embree4_sys::RTCBuildQuality::HIGH,
            flags: embree4_sys::RTCSceneFlags::ROBUST,
        },
    )?;

    
    let num_tris = 1_000_000;
    let mut mesh = Mesh::new_tri_capacity(num_tris);
    mesh.material.color = LinearRgba::rgb(1.0, 0.0, 0.0);
    // mesh.verts.push((0.0, 0.5, 1.0));
    // mesh.verts.push((1.0, -0.5, 1.0));
    // mesh.verts.push((-1.0, -0.5, 1.0));
    mesh.verts.push((0.0, 50.0, 1.0));
    mesh.verts.push((100.0, -50.0, 1.0));
    mesh.verts.push((-100.0, -50.0, 1.0));
    mesh.indices.push((0, 1, 2));

    let mut storage = MeshStorage::default();
    let _mesh_id: u32 = storage.attach(mesh, &device, &mut scene)?;

    let mut commited_scene = scene.commit()?;

    camera.render(&mut image, &mut commited_scene, &storage, 20);

    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
