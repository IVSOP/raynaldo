use bevy_color::LinearRgba;
use glam::Vec3;
use crate::mesh::*;
use embree4_rs::*;
use anyhow::*;
use crate::common::*;

pub fn cornell_box<'a>(meshes: &mut MeshStorage, lights: &mut LightStorage, device: &Device, mut scene: &mut Scene<'a>) -> Result<u32> {

    let white_material = Material {
        color: LinearRgba::rgb(0.9, 0.9, 0.9),
        diffuse: LinearRgba::rgb(0.4, 0.4, 0.4),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        ..default()
    };
    let red_material = Material {
        color: LinearRgba::rgb(0.9, 0.0, 0.0),
        diffuse: LinearRgba::rgb(0.4, 0.0, 0.0),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        ..default()
    };
    let green_material = Material {
        color: LinearRgba::rgb(0.0, 0.9, 0.0),
        diffuse: LinearRgba::rgb(0.0, 0.2, 0.0),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        ..default()
    };
    let blue_material = Material {
        color: LinearRgba::rgb(0.0, 0.0, 0.9),
        diffuse: LinearRgba::rgb(0.0, 0.0, 0.4),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        ..default()
    };
    let orange_material = Material {
        color: LinearRgba::rgb(0.99, 0.65, 0.0),
        diffuse: LinearRgba::rgb(0.37, 0.24, 0.0),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        ..default()
    };
    let mirror_material = Material {
        color: LinearRgba::BLACK,
        diffuse: LinearRgba::BLACK,
        specular: LinearRgba::rgb(0.9, 0.9, 0.9),
        transmission: LinearRgba::BLACK,
        ..default()
    };
    let glass_material = Material {
        color: LinearRgba::BLACK,
        diffuse: LinearRgba::BLACK,
        specular: LinearRgba::rgb(0.2, 0.2, 0.2),
        transmission: LinearRgba::rgb(0.9, 0.9, 0.9),
        refraction: 1.2,
    };
    
    
    let mut ceiling = Mesh::with_material(white_material.clone());
    ceiling.verts.push((556.0, 548.8, 0.0));
    ceiling.verts.push((0.0, 548.8, 0.0));
    ceiling.verts.push((0.0, 548.8, 559.2));
    ceiling.verts.push((556.0, 548.8, 559.2));
    ceiling.indices.push((0, 2, 1));
    ceiling.indices.push((0, 3, 2));
    
    let mut floor = Mesh::with_material(white_material.clone());
    floor.verts.push((552.8, 0.0, 0.0));
    floor.verts.push((0.0, 0.0, 0.0));
    floor.verts.push((0.0, 0.0, 559.2));
    floor.verts.push((549.6, 0.0, 559.2));
    floor.indices.push((0, 1, 2));
    floor.indices.push((0, 2, 3));
    
    let mut back = Mesh::with_material(white_material.clone());
    back.verts.push((0.0, 0.0, 559.2));
    back.verts.push((549.6, 0.0, 559.2));
    back.verts.push((556.0, 548.8, 559.2));
    back.verts.push((0.0, 548.8, 559.2));
    back.indices.push((2, 1, 0));
    back.indices.push((3, 2, 0));
    
    let mut left = Mesh::with_material(green_material.clone());
    left.verts.push((0.0, 0.0, 0.0));
    left.verts.push((0., 0., 559.2));
    left.verts.push((0., 548.8, 559.2));
    left.verts.push((0., 548.8, 0.));
    left.indices.push((0, 2, 1));
    left.indices.push((0, 3, 2));
    
    let mut right = Mesh::with_material(red_material.clone());
    right.verts.push((552.8, 0.0, 0.));
    right.verts.push((549.6, 0., 559.2));
    right.verts.push((549.6, 548.8, 559.2));
    right.verts.push((552.8, 548.8, 0.));
    right.indices.push((0, 1, 2));
    right.indices.push((0, 2, 3));
    
    let mut short_block_top = Mesh::with_material(orange_material.clone());
    short_block_top.verts.push((130.0, 165.0,  65.0));
    short_block_top.verts.push((82.0, 165.0, 225.0));
    short_block_top.verts.push((240.0, 165.0, 272.0));
    short_block_top.verts.push((290.0, 165.0, 114.0));
    short_block_top.indices.push((0, 1, 2));
    short_block_top.indices.push((0, 2, 3));
    
    let mut short_block_bot = Mesh::with_material(orange_material.clone());
    short_block_bot.verts.push((130.0, 0.01,  65.0));
    short_block_bot.verts.push((82.0, 0.01, 225.0));
    short_block_bot.verts.push((240.0, 0.01, 272.0));
    short_block_bot.verts.push((290.0, 0.01, 114.0));
    short_block_bot.indices.push((0, 1, 2));
    short_block_bot.indices.push((0, 2, 3));
    
    let mut short_block_left = Mesh::with_material(orange_material.clone());
    short_block_left.verts.push((290.0, 0.0, 114.0));
    short_block_left.verts.push((290.0, 165.0, 114.0));
    short_block_left.verts.push((240.0, 165.0, 272.0));
    short_block_left.verts.push((240.0,  0.0, 272.0));
    short_block_left.indices.push((0, 1, 2));
    short_block_left.indices.push((0, 2, 3));
    
    let mut short_block_back = Mesh::with_material(orange_material.clone());
    short_block_back.verts.push((240.0, 0.0, 272.0));
    short_block_back.verts.push((240.0, 165.0, 272.0));
    short_block_back.verts.push((82.0, 165., 225.0));
    short_block_back.verts.push((82.0, 0.0, 225.0));
    short_block_back.indices.push((0, 1, 2));
    short_block_back.indices.push((0, 2, 3));
    
    let mut short_block_right = Mesh::with_material(orange_material.clone());
    short_block_right.verts.push((82.0, 0.0, 225.0));
    short_block_right.verts.push((82.0, 165.0, 225.0));
    short_block_right.verts.push((130.0, 165.0, 65.0));
    short_block_right.verts.push((130.0, 0.0, 65.0));
    short_block_right.indices.push((0, 1, 2));
    short_block_right.indices.push((0, 2, 3));
    
    let mut short_block_front = Mesh::with_material(orange_material.clone());
    short_block_front.verts.push((130.0, 0.0, 65.0));
    short_block_front.verts.push((130.0, 165.0, 65.0));
    short_block_front.verts.push((290.0, 165.0, 114.0));
    short_block_front.verts.push((290.0, 0.0, 114.0));
    short_block_front.indices.push((0, 1, 2));
    short_block_front.indices.push((0, 2, 3));
    
    
    let mut tall_block_top = Mesh::with_material(blue_material.clone());
    tall_block_top.verts.push((423.0, 330.0, 247.0));
    tall_block_top.verts.push((265.0, 330.0, 296.0));
    tall_block_top.verts.push((314.0, 330.0, 456.0));
    tall_block_top.verts.push((472.0, 330.0, 406.0));
    tall_block_top.indices.push((0, 1, 2));
    tall_block_top.indices.push((0, 2, 3));
    
    let mut tall_block_bot = Mesh::with_material(blue_material.clone());
    tall_block_bot.verts.push((423.0, 0.1, 247.0));
    tall_block_bot.verts.push((265.0, 0.1, 296.0));
    tall_block_bot.verts.push((314.0, 0.1, 456.0));
    tall_block_bot.verts.push((472.0, 0.1, 406.0));
    tall_block_bot.indices.push((0, 1, 2));
    tall_block_bot.indices.push((0, 2, 3));
    
    let mut tall_block_left = Mesh::with_material(blue_material.clone());
    tall_block_left.verts.push((423.0, 0.0, 247.0));
    tall_block_left.verts.push((423.0, 330.0, 247.0));
    tall_block_left.verts.push((472.0, 330.0, 406.0));
    tall_block_left.verts.push((472.0, 0.0, 406.0));
    tall_block_left.indices.push((0, 1, 2));
    tall_block_left.indices.push((0, 2, 3));
    
    let mut tall_block_back = Mesh::with_material(blue_material.clone());
    tall_block_back.verts.push((472.0, 330.0, 406.0));
    tall_block_back.verts.push((472.0, 330.0, 406.0));
    tall_block_back.verts.push((314.0, 330.0, 456.0));
    tall_block_back.verts.push((314.0, 0.0, 406.0));
    tall_block_back.indices.push((0, 1, 2));
    tall_block_back.indices.push((0, 2, 3));
    
    let mut tall_block_right = Mesh::with_material(blue_material.clone());
    tall_block_right.verts.push((314.0, 0.0, 456.0));
    tall_block_right.verts.push((314.0, 330.0, 456.0));
    tall_block_right.verts.push((265.0, 330.0, 296.0));
    tall_block_right.verts.push((265.0, 0.0, 296.0));
    tall_block_right.indices.push((0, 1, 2));
    tall_block_right.indices.push((0, 2, 3));
    
    let mut tall_block_front = Mesh::with_material(blue_material.clone());
    tall_block_front.verts.push((265.0, 0.0, 296.0));
    tall_block_front.verts.push((265.0, 330.0, 296.0));
    tall_block_front.verts.push((423.0, 330.0, 247.0));
    tall_block_front.verts.push((423.0, 0.0, 247.0));
    tall_block_front.indices.push((0, 1, 2));
    tall_block_front.indices.push((0, 2, 3));

    let mut mirror = Mesh::with_material(mirror_material.clone());
    mirror.verts.push((552.0, 50.0, 50.));
    mirror.verts.push((549.0, 50.0, 509.2));
    mirror.verts.push((549.0, 488.8, 509.2));
    mirror.verts.push((552.0, 488.8, 50.0));
    mirror.indices.push((0, 1, 2));
    mirror.indices.push((0, 2, 3));


    let mut total = 0;
    let _mesh_id: u32 = meshes.attach(ceiling, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(floor, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(back, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(left, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(right, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(short_block_top, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(short_block_bot, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(short_block_right, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(short_block_left, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(short_block_back, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(short_block_front, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(tall_block_top, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(tall_block_bot, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(tall_block_right, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(tall_block_left, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(tall_block_back, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(tall_block_front, &device, &mut scene)?; total += 1;
    let _mesh_id: u32 = meshes.attach(mirror, &device, &mut scene)?; total += 1;

    let ambient = Light {
        light_type: LightType::AMBIENT,
        color: LinearRgba::rgb(0.07, 0.07, 0.07)
        // color: LinearRgba::rgb(1.0, 1.0, 1.0)
    };
    lights.lights.push(ambient);

    let n_points_dim = 3; // must be 1, 3 or 5
    let n_half: i32 = (n_points_dim - 1) / 2;
    for x in -n_half..(n_half + 1) {
        for z in -n_half..(n_half + 1) {
            let power = 1.0 / ((n_points_dim * n_points_dim) as f32);
            let point_light = Light {
                light_type: LightType::POINT(Vec3::new(278.0 + (x as f32 * 100.0), 545.0, 280.0 + (z as f32 * 100.0))),
                color: LinearRgba::rgb(power, power, power),
            };
            lights.lights.push(point_light);
        }
    }

    // lights.lights.push(
    //     Light {
    //         light_type: LightType::POINT(Vec3::new(378.0, 545.0, 380.0)),
    //         color: LinearRgba::rgb(0.1, 0.1, 0.1),
    //     }
    // );

    Ok(total)
}
