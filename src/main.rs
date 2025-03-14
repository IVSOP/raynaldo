use anyhow::Context;
use image::buffer::ConvertBuffer;
use image::{Rgb32FImage, RgbImage};
use embree4_rs::*;
use glam::*;
use mesh::{Mesh, MeshStorage};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

mod common;
use common::*;

mod camera;
use camera::*;

mod ray;

mod mesh;

const W: u32 = 640;
const H: u32 = 640;

fn main() -> anyhow::Result<()> {
    let mut image = Rgb32FImage::new(W, H);

    for pixel in image.pixels_mut() {
        pixel[0] = 0.4;
        pixel[1] = 0.4;
        pixel[2] = 0.4;
    }

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
    mesh.verts.push((0.0, 0.5, 1.0));
    mesh.verts.push((1.0, -0.5, 1.0));
    mesh.verts.push((-1.0, -0.5, 1.0));
    mesh.indices.push((0, 1, 2));

    let mut storage = MeshStorage::default();
    let mesh_id: u32 = storage.attach(mesh, &device, &mut scene)?;





    let scene = scene.commit()?;

    
    // // for i in 0..num_tris as u32 {
    // //     let pos = 1_000.0 * (2.0 * rand_dir() - 1.0);

    // //     let p = pos + rand_dir();
    // //     let q = pos + rand_dir();
    // //     let r = pos + rand_dir();

    // //     vertices.push((p.x, p.y, p.z));
    // //     vertices.push((q.x, q.y, q.z));
    // //     vertices.push((r.x, r.y, r.z));

    // //     indices.push((3 * i, 3 * i + 1, 3 * i + 2));
    // // }


    let num_rays = 1_000_000;
    let rays: Vec<_> = (0..num_rays)
        .map(|_| {
            let origin = Vec3::ZERO;
            let direction = Vec3::new(0.0, 0.0, 1.0);

            embree4_sys::RTCRay {
                org_x: origin.x,
                org_y: origin.y,
                org_z: origin.z,
                dir_x: direction.x,
                dir_y: direction.y,
                dir_z: direction.z,
                ..Default::default()
            }
        })
        .collect();

    let t0 = std::time::Instant::now();
    let hits: usize = rays
        .into_par_iter()
        .map(|ray| match scene.intersect_1(ray).unwrap() {
            Some(hit) => {
                // println!("{:#?}", hit);
                let origin = Vec3::new(hit.ray.org_x, hit.ray.org_y, hit.ray.org_z);
                let dir = Vec3::new(hit.ray.dir_x, hit.ray.dir_y, hit.ray.dir_z);
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! A NORMAL NAO VEM NECESSARIAMENTE NORMALIZADA
                let normal = Vec3::new(hit.hit.Ng_x, hit.hit.Ng_y, hit.hit.Ng_z);
                let mesh: &Mesh = storage.get(hit.hit.geomID).unwrap();
                println!("hit at {} with normal {} and color {}", origin + (dir * hit.ray.tfar), normal, mesh.material.color);
                1
            },
            None => 0,
        })
        .sum();
    let elapsed = t0.elapsed();
    let rays_per_sec = (num_rays as f32 / elapsed.as_secs_f32()) as usize;

    println!("Traced {} rays in {:?}", num_rays, elapsed);
    let frac_hits = hits as f32 / num_rays as f32;
    println!("  {} hits ({:.3}%)", hits, 100.0 * frac_hits);
    println!("  ({} rays/s)", rays_per_sec);


    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
