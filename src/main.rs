use crate::consts::Consts;
use anyhow::Context;
use bevy_math::{Quat, Vec3};
use bevy_transform::components::Transform;
use image::RgbImage;
use image::buffer::ConvertBuffer;

mod common;
mod cornell;

mod camera;

mod consts;
mod geometry;
mod tonemap;

fn main() -> anyhow::Result<()> {
    // let args: Vec<String> = env::args().collect();

    let camera = camera::Camera::new(
        Consts::CAM_POS,
        Consts::CAM_LOOKAT,
        Vec3::Y,
        Consts::W,
        Consts::H,
        Consts::CAM_FOV,
    );

    let device = embree4_rs::Device::try_new(None)?;
    let mut scene = embree4_rs::Scene::try_new(
        &device,
        embree4_rs::SceneOptions {
            build_quality: embree4_sys::RTCBuildQuality::HIGH,
            flags: embree4_sys::RTCSceneFlags::ROBUST,
        },
    )?;

    let mut store = geometry::SceneStorage::new()?;
    store.load_textures_batch(&vec![
        "assets/textures/skybox/front.jpg",
        "assets/textures/skybox/back.jpg",
        "assets/textures/skybox/right.jpg",
        "assets/textures/skybox/left.jpg",
        "assets/textures/skybox/top.jpg",
        "assets/textures/skybox/bottom.jpg",
    ])?;
    cornell::cornell_box(&mut store, &device, &mut scene)?;

    let (gltf_doc, gltf_buff, _) = gltf::import("assets/magujo/suzanne.glb")?;
    let transform = Transform {
        translation: Vec3::new(450.0, 50.0, 150.0),
        rotation: Quat::from_rotation_y(220.0_f32.to_radians()),
        scale: Vec3::splat(50.0),
    };
    cornell::add_gltf(
        &mut store,
        &device,
        &mut scene,
        &gltf_doc,
        &gltf_buff,
        transform.compute_matrix(),
        &geometry::Material::MIRROR_MATERIAL,
    )?;

    cornell::add_skybox(&mut store, &device, &mut scene)?;

    let mut commited_scene = scene.commit()?;
    let mut image = camera.render(&mut commited_scene, &store);

    tonemap::tonemap(&mut image);
    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
