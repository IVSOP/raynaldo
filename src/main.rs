use anyhow::Context;
use image::buffer::ConvertBuffer;
use image::{Rgb32FImage, RgbImage};
use embree4_rs::*;
use glam::*;
use mesh::{Material, Mesh, MeshStorage};
use bevy_color::LinearRgba;

mod common;
// use common::*;

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

    let white_material = Material::color(LinearRgba::rgb(0.9, 0.9, 0.9));
    let red_material = Material::color(LinearRgba::rgb(0.9, 0.0, 0.0));
    let green_material = Material::color(LinearRgba::rgb(0.0, 0.9, 0.0));
    let blue_material = Material::color(LinearRgba::rgb(0.0, 0.0, 0.9));
    let orange_material = Material::color(LinearRgba::rgb(0.99, 0.65, 0.));


    let mut ceiling = Mesh::with_material(white_material.clone());
    ceiling.verts.push((556.0, 548.8, 0.0));
    ceiling.verts.push((0.0, 548.8, 0.0));
    ceiling.verts.push((0.0, 548.8, 559.2));
    ceiling.verts.push((556.0, 548.8, 559.2));
    ceiling.indices.push((0, 1, 2));
    ceiling.indices.push((0, 2, 3));

    let mut floor = Mesh::with_material(white_material.clone());
    floor.verts.push((552.8, 0.0, 0.0));
    floor.verts.push((0.0, 0.0, 0.0));
    floor.verts.push((0.0, 0.0, 559.2));
    floor.verts.push((549.6, 0.0, 559.2));
    floor.indices.push((0, 1, 2));
    floor.indices.push((3, 0, 2));

    let mut back = Mesh::with_material(white_material.clone());
    back.verts.push((0.0, 0.0, 559.2));
    back.verts.push((549.6, 0.0, 559.2));
    back.verts.push((556.0, 548.8, 559.2));
    back.verts.push((0.0, 548.8, 559.2));
    back.indices.push((0, 1, 2));
    back.indices.push((0, 3, 2));

    let mut left = Mesh::with_material(green_material.clone());
    left.verts.push((0.0, 0.0, 0.0));
    left.verts.push((0., 0., 559.2));
    left.verts.push((0., 548.8, 559.2));
    left.verts.push((0., 548.8, 0.));
    left.indices.push((0, 1, 2));
    left.indices.push((0, 3, 2));

    let mut right = Mesh::with_material(red_material.clone());
    right.verts.push((552.8, 0.0, 0.));
    right.verts.push((549.6, 0., 559.2));
    right.verts.push((549.6, 548.8, 559.2));
    right.verts.push((552.8, 548.8, 0.));
    right.indices.push((0, 1, 2));
    right.indices.push((0, 3, 2));

    let mut short_block_top = Mesh::with_material(orange_material.clone());
    short_block_top.verts.push((130.0, 165.0,  65.0));
    short_block_top.verts.push((82.0, 165.0, 225.0));
    short_block_top.verts.push((240.0, 165.0, 272.0));
    short_block_top.verts.push((290.0, 165.0, 114.0));
    short_block_top.indices.push((0, 1, 2));
    short_block_top.indices.push((0, 3, 2));

    let mut short_block_bot = Mesh::with_material(orange_material.clone());
    short_block_bot.verts.push((130.0, 0.01,  65.0));
    short_block_bot.verts.push((82.0, 0.01, 225.0));
    short_block_bot.verts.push((240.0, 0.01, 272.0));
    short_block_bot.verts.push((290.0, 0.01, 114.0));
    short_block_bot.indices.push((0, 1, 2));
    short_block_bot.indices.push((0, 3, 2));

    let mut short_block_left = Mesh::with_material(orange_material.clone());
    short_block_left.verts.push((290.0, 0.0, 114.0));
    short_block_left.verts.push((290.0, 165.0, 114.0));
    short_block_left.verts.push((240.0, 165.0, 272.0));
    short_block_left.verts.push((240.0,  0.0, 272.0));
    short_block_left.indices.push((0, 1, 2));
    short_block_left.indices.push((0, 3, 2));

    let mut short_block_back = Mesh::with_material(orange_material.clone());
    short_block_back.verts.push((240.0, 0.0, 272.0));
    short_block_back.verts.push((240.0, 165.0, 272.0));
    short_block_back.verts.push((82.0, 165., 225.0));
    short_block_back.verts.push((82.0, 0.0, 225.0));
    short_block_back.indices.push((0, 1, 2));
    short_block_back.indices.push((0, 3, 2));

    let mut short_block_right = Mesh::with_material(orange_material.clone());
    short_block_right.verts.push((82.0, 0.0, 225.0));
    short_block_right.verts.push((82.0, 165.0, 225.0));
    short_block_right.verts.push((130.0, 165.0, 65.0));
    short_block_right.verts.push((130.0, 0.0, 65.0));
    short_block_right.indices.push((0, 1, 2));
    short_block_right.indices.push((0, 3, 2));

    let mut short_block_front = Mesh::with_material(orange_material.clone());
    short_block_front.verts.push((130.0, 0.0, 65.0));
    short_block_front.verts.push((130.0, 165.0, 65.0));
    short_block_front.verts.push((290.0, 165.0, 114.0));
    short_block_front.verts.push((290.0, 0.0, 114.0));
    short_block_front.indices.push((0, 1, 2));
    short_block_front.indices.push((0, 3, 2));


    let mut tall_block_top = Mesh::with_material(blue_material.clone());
    tall_block_top.verts.push((423.0, 330.0, 247.0));
    tall_block_top.verts.push((265.0, 330.0, 296.0));
    tall_block_top.verts.push((314.0, 330.0, 456.0));
    tall_block_top.verts.push((472.0, 330.0, 406.0));
    tall_block_top.indices.push((0, 1, 2));
    tall_block_top.indices.push((0, 3, 2));

    let mut tall_block_bot = Mesh::with_material(blue_material.clone());
    tall_block_bot.verts.push((423.0, 0.1, 247.0));
    tall_block_bot.verts.push((265.0, 0.1, 296.0));
    tall_block_bot.verts.push((314.0, 0.1, 456.0));
    tall_block_bot.verts.push((472.0, 0.1, 406.0));
    tall_block_bot.indices.push((0, 1, 2));
    tall_block_bot.indices.push((0, 3, 2));

    let mut tall_block_left = Mesh::with_material(blue_material.clone());
    tall_block_left.verts.push((423.0, 0.0, 247.0));
    tall_block_left.verts.push((423.0, 330.0, 247.0));
    tall_block_left.verts.push((472.0, 330.0, 406.0));
    tall_block_left.verts.push((472.0, 0.0, 406.0));
    tall_block_left.indices.push((0, 1, 2));
    tall_block_left.indices.push((0, 3, 2));

    let mut tall_block_back = Mesh::with_material(blue_material.clone());
    tall_block_back.verts.push((472.0, 330.0, 406.0));
    tall_block_back.verts.push((472.0, 330.0, 406.0));
    tall_block_back.verts.push((314.0, 330.0, 456.0));
    tall_block_back.verts.push((314.0, 0.0, 406.0));
    tall_block_back.indices.push((0, 1, 2));
    tall_block_back.indices.push((0, 3, 2));

    let mut tall_block_right = Mesh::with_material(blue_material.clone());
    tall_block_right.verts.push((314.0, 0.0, 456.0));
    tall_block_right.verts.push((314.0, 330.0, 456.0));
    tall_block_right.verts.push((265.0, 330.0, 296.0));
    tall_block_right.verts.push((265.0, 0.0, 296.0));
    tall_block_right.indices.push((0, 1, 2));
    tall_block_right.indices.push((0, 3, 2));

    let mut tall_block_front = Mesh::with_material(blue_material.clone());
    tall_block_front.verts.push((265.0, 0.0, 296.0));
    tall_block_front.verts.push((265.0, 330.0, 296.0));
    tall_block_front.verts.push((423.0, 330.0, 247.0));
    tall_block_front.verts.push((423.0, 0.0, 247.0));
    tall_block_front.indices.push((0, 1, 2));
    tall_block_front.indices.push((0, 3, 2));

    let mut storage = MeshStorage::default();
    let _mesh_id: u32 = storage.attach(ceiling, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(floor, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(back, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(left, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(right, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(short_block_top, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(short_block_bot, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(short_block_right, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(short_block_left, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(short_block_back, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(short_block_front, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(tall_block_top, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(tall_block_bot, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(tall_block_right, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(tall_block_left, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(tall_block_back, &device, &mut scene)?;
    let _mesh_id: u32 = storage.attach(tall_block_front, &device, &mut scene)?;

    let mut commited_scene = scene.commit()?;

    camera.render(&mut image, &mut commited_scene, &storage, 20);

    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
