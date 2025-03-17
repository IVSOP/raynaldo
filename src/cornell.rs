// use crate::common::*;
use crate::geometry::*;
use anyhow::*;
use bevy_color::LinearRgba;
use bevy_math::*;
use bevy_transform::components::*;
use embree4_rs::*;
use gltf::{
    // Gltf,
    // buffer::Data,
    mesh::util::*,
};

pub const WHITE_MATERIAL: Material = Material {
    color: LinearRgba::rgb(0.9, 0.9, 0.9),
    diffuse: LinearRgba::rgb(0.4, 0.4, 0.4),
    specular: LinearRgba::BLACK,
    transmission: LinearRgba::BLACK,
    refraction: 1.0,
    reflectivity: 0.0,
    transparency: 0.0,
};

pub const RED_MATERIAL: Material = Material {
    color: LinearRgba::rgb(0.9, 0.0, 0.0),
    diffuse: LinearRgba::rgb(0.4, 0.0, 0.0),
    specular: LinearRgba::BLACK,
    transmission: LinearRgba::BLACK,
    refraction: 1.0,
    reflectivity: 0.0,
    transparency: 0.0,
};
pub const GREEN_MATERIAL: Material = Material {
    color: LinearRgba::rgb(0.0, 0.9, 0.0),
    diffuse: LinearRgba::rgb(0.0, 0.2, 0.0),
    specular: LinearRgba::BLACK,
    transmission: LinearRgba::BLACK,
    refraction: 1.0,
    reflectivity: 0.0,
    transparency: 0.0,
};
pub const BLUE_MATERIAL: Material = Material {
    color: LinearRgba::rgb(0.0, 0.0, 0.9),
    diffuse: LinearRgba::rgb(0.0, 0.0, 0.4),
    specular: LinearRgba::BLACK,
    transmission: LinearRgba::BLACK,
    refraction: 1.0,
    reflectivity: 0.0,
    transparency: 0.0,
};
pub const ORANGE_MATERIAL: Material = Material {
    color: LinearRgba::rgb(0.99, 0.65, 0.0),
    diffuse: LinearRgba::rgb(0.37, 0.24, 0.0),
    specular: LinearRgba::BLACK,
    transmission: LinearRgba::BLACK,
    refraction: 1.0,
    reflectivity: 0.0,
    transparency: 0.0,
};
pub const MIRROR_MATERIAL: Material = Material {
    color: LinearRgba::BLACK,
    diffuse: LinearRgba::BLACK,
    specular: LinearRgba::rgb(0.9, 0.9, 0.9),
    transmission: LinearRgba::BLACK,
    refraction: 1.5,
    reflectivity: 1.0,
    transparency: 0.0,
};
pub const GLASS_MATERIAL: Material = Material {
    color: LinearRgba::WHITE,
    diffuse: LinearRgba::BLACK,
    specular: LinearRgba::rgb(1.0, 1.0, 1.0),
    transmission: LinearRgba::rgb(0.9, 0.9, 0.9),
    refraction: 1.125,
    reflectivity: 0.1, // try 0.01
    transparency: 1.0,
};

pub fn cornell_box(
    geom: &mut GeomStorage,
    lights: &mut LightStorage,
    device: &Device,
    mut scene: &mut Scene<'_>,
) -> Result<u32> {
    let mut ceiling_mesh = Mesh::default();
    ceiling_mesh.verts.push((556.0, 548.8, 0.0));
    ceiling_mesh.verts.push((0.0, 548.8, 0.0));
    ceiling_mesh.verts.push((0.0, 548.8, 559.2));
    ceiling_mesh.verts.push((556.0, 548.8, 559.2));
    ceiling_mesh.indices.push((0, 2, 1));
    ceiling_mesh.indices.push((0, 3, 2));
    let ceiling = Geometry::with_material(WHITE_MATERIAL, GeomInfo::MESH(ceiling_mesh));

    let mut floor_mesh = Mesh::default();
    floor_mesh.verts.push((552.8, 0.0, 0.0));
    floor_mesh.verts.push((0.0, 0.0, 0.0));
    floor_mesh.verts.push((0.0, 0.0, 559.2));
    floor_mesh.verts.push((549.6, 0.0, 559.2));
    floor_mesh.indices.push((0, 1, 2));
    floor_mesh.indices.push((0, 2, 3));
    let floor = Geometry::with_material(WHITE_MATERIAL, GeomInfo::MESH(floor_mesh));

    let mut back_mesh = Mesh::default();
    back_mesh.verts.push((0.0, 0.0, 559.2));
    back_mesh.verts.push((549.6, 0.0, 559.2));
    back_mesh.verts.push((556.0, 548.8, 559.2));
    back_mesh.verts.push((0.0, 548.8, 559.2));
    back_mesh.indices.push((2, 1, 0));
    back_mesh.indices.push((3, 2, 0));
    let back = Geometry::with_material(WHITE_MATERIAL, GeomInfo::MESH(back_mesh));

    let mut left_mesh = Mesh::default();
    left_mesh.verts.push((0.0, 0.0, 0.0));
    left_mesh.verts.push((0., 0., 559.2));
    left_mesh.verts.push((0., 548.8, 559.2));
    left_mesh.verts.push((0., 548.8, 0.));
    left_mesh.indices.push((0, 2, 1));
    left_mesh.indices.push((0, 3, 2));
    let left = Geometry::with_material(GREEN_MATERIAL, GeomInfo::MESH(left_mesh));

    let mut right_mesh = Mesh::default();
    right_mesh.verts.push((552.8, 0.0, 0.));
    right_mesh.verts.push((549.6, 0., 559.2));
    right_mesh.verts.push((549.6, 548.8, 559.2));
    right_mesh.verts.push((552.8, 548.8, 0.));
    right_mesh.indices.push((0, 1, 2));
    right_mesh.indices.push((0, 2, 3));
    let right = Geometry::with_material(RED_MATERIAL, GeomInfo::MESH(right_mesh));

    let mut short_block_top_mesh = Mesh::default();
    short_block_top_mesh.verts.push((130.0, 165.0, 65.0));
    short_block_top_mesh.verts.push((82.0, 165.0, 225.0));
    short_block_top_mesh.verts.push((240.0, 165.0, 272.0));
    short_block_top_mesh.verts.push((290.0, 165.0, 114.0));
    short_block_top_mesh.indices.push((0, 1, 2));
    short_block_top_mesh.indices.push((0, 2, 3));
    let short_block_top =
        Geometry::with_material(ORANGE_MATERIAL, GeomInfo::MESH(short_block_top_mesh));

    let mut short_block_bot_mesh = Mesh::default();
    short_block_bot_mesh.verts.push((130.0, 0.01, 65.0));
    short_block_bot_mesh.verts.push((82.0, 0.01, 225.0));
    short_block_bot_mesh.verts.push((240.0, 0.01, 272.0));
    short_block_bot_mesh.verts.push((290.0, 0.01, 114.0));
    short_block_bot_mesh.indices.push((0, 1, 2));
    short_block_bot_mesh.indices.push((0, 2, 3));
    let short_block_bot =
        Geometry::with_material(ORANGE_MATERIAL, GeomInfo::MESH(short_block_bot_mesh));

    let mut short_block_left_mesh = Mesh::default();
    short_block_left_mesh.verts.push((290.0, 0.0, 114.0));
    short_block_left_mesh.verts.push((290.0, 165.0, 114.0));
    short_block_left_mesh.verts.push((240.0, 165.0, 272.0));
    short_block_left_mesh.verts.push((240.0, 0.0, 272.0));
    short_block_left_mesh.indices.push((0, 1, 2));
    short_block_left_mesh.indices.push((0, 2, 3));
    let short_block_left =
        Geometry::with_material(ORANGE_MATERIAL, GeomInfo::MESH(short_block_left_mesh));

    let mut short_block_back_mesh = Mesh::default();
    short_block_back_mesh.verts.push((240.0, 0.0, 272.0));
    short_block_back_mesh.verts.push((240.0, 165.0, 272.0));
    short_block_back_mesh.verts.push((82.0, 165., 225.0));
    short_block_back_mesh.verts.push((82.0, 0.0, 225.0));
    short_block_back_mesh.indices.push((0, 1, 2));
    short_block_back_mesh.indices.push((0, 2, 3));
    let short_block_back =
        Geometry::with_material(ORANGE_MATERIAL, GeomInfo::MESH(short_block_back_mesh));

    let mut short_block_right_mesh = Mesh::default();
    short_block_right_mesh.verts.push((82.0, 0.0, 225.0));
    short_block_right_mesh.verts.push((82.0, 165.0, 225.0));
    short_block_right_mesh.verts.push((130.0, 165.0, 65.0));
    short_block_right_mesh.verts.push((130.0, 0.0, 65.0));
    short_block_right_mesh.indices.push((0, 1, 2));
    short_block_right_mesh.indices.push((0, 2, 3));
    let short_block_right =
        Geometry::with_material(ORANGE_MATERIAL, GeomInfo::MESH(short_block_right_mesh));

    let mut short_block_front_mesh = Mesh::default();
    short_block_front_mesh.verts.push((130.0, 0.0, 65.0));
    short_block_front_mesh.verts.push((130.0, 165.0, 65.0));
    short_block_front_mesh.verts.push((290.0, 165.0, 114.0));
    short_block_front_mesh.verts.push((290.0, 0.0, 114.0));
    short_block_front_mesh.indices.push((0, 1, 2));
    short_block_front_mesh.indices.push((0, 2, 3));
    let short_block_front =
        Geometry::with_material(ORANGE_MATERIAL, GeomInfo::MESH(short_block_front_mesh));

    let mut tall_block_top_mesh = Mesh::default();
    tall_block_top_mesh.verts.push((423.0, 330.0, 247.0));
    tall_block_top_mesh.verts.push((265.0, 330.0, 296.0));
    tall_block_top_mesh.verts.push((314.0, 330.0, 456.0));
    tall_block_top_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_top_mesh.indices.push((0, 1, 2));
    tall_block_top_mesh.indices.push((0, 2, 3));
    let tall_block_top =
        Geometry::with_material(BLUE_MATERIAL, GeomInfo::MESH(tall_block_top_mesh));

    let mut tall_block_bot_mesh = Mesh::default();
    tall_block_bot_mesh.verts.push((423.0, 0.1, 247.0));
    tall_block_bot_mesh.verts.push((265.0, 0.1, 296.0));
    tall_block_bot_mesh.verts.push((314.0, 0.1, 456.0));
    tall_block_bot_mesh.verts.push((472.0, 0.1, 406.0));
    tall_block_bot_mesh.indices.push((0, 1, 2));
    tall_block_bot_mesh.indices.push((0, 2, 3));
    let tall_block_bot =
        Geometry::with_material(BLUE_MATERIAL, GeomInfo::MESH(tall_block_bot_mesh));

    let mut tall_block_left_mesh = Mesh::default();
    tall_block_left_mesh.verts.push((423.0, 0.0, 247.0));
    tall_block_left_mesh.verts.push((423.0, 330.0, 247.0));
    tall_block_left_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_left_mesh.verts.push((472.0, 0.0, 406.0));
    tall_block_left_mesh.indices.push((0, 1, 2));
    tall_block_left_mesh.indices.push((0, 2, 3));
    let tall_block_left =
        Geometry::with_material(BLUE_MATERIAL, GeomInfo::MESH(tall_block_left_mesh));

    let mut tall_block_back_mesh = Mesh::default();
    tall_block_back_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_back_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_back_mesh.verts.push((314.0, 330.0, 456.0));
    tall_block_back_mesh.verts.push((314.0, 0.0, 406.0));
    tall_block_back_mesh.indices.push((0, 1, 2));
    tall_block_back_mesh.indices.push((0, 2, 3));
    let tall_block_back =
        Geometry::with_material(BLUE_MATERIAL, GeomInfo::MESH(tall_block_back_mesh));

    let mut tall_block_right_mesh = Mesh::default();
    tall_block_right_mesh.verts.push((314.0, 0.0, 456.0));
    tall_block_right_mesh.verts.push((314.0, 330.0, 456.0));
    tall_block_right_mesh.verts.push((265.0, 330.0, 296.0));
    tall_block_right_mesh.verts.push((265.0, 0.0, 296.0));
    tall_block_right_mesh.indices.push((0, 1, 2));
    tall_block_right_mesh.indices.push((0, 2, 3));
    let tall_block_right =
        Geometry::with_material(BLUE_MATERIAL, GeomInfo::MESH(tall_block_right_mesh));

    let mut tall_block_front_mesh = Mesh::default();
    tall_block_front_mesh.verts.push((265.0, 0.0, 296.0));
    tall_block_front_mesh.verts.push((265.0, 330.0, 296.0));
    tall_block_front_mesh.verts.push((423.0, 330.0, 247.0));
    tall_block_front_mesh.verts.push((423.0, 0.0, 247.0));
    tall_block_front_mesh.indices.push((0, 1, 2));
    tall_block_front_mesh.indices.push((0, 2, 3));
    let tall_block_front =
        Geometry::with_material(BLUE_MATERIAL, GeomInfo::MESH(tall_block_front_mesh));

    let mut mirror_mesh = Mesh::default();
    mirror_mesh.verts.push((552.0, 50.0, 50.));
    mirror_mesh.verts.push((549.0, 50.0, 509.2));
    mirror_mesh.verts.push((549.0, 488.8, 509.2));
    mirror_mesh.verts.push((552.0, 488.8, 50.0));
    mirror_mesh.indices.push((0, 1, 2));
    mirror_mesh.indices.push((0, 2, 3));
    let mirror = Geometry::with_material(MIRROR_MATERIAL, GeomInfo::MESH(mirror_mesh));

    let sphere = Sphere {
        radius: 110.0,
        center: Vec3::new(160.0, 320.0, 225.0),
    };
    let sphere_geometry = Geometry::with_material(GLASS_MATERIAL, GeomInfo::SPHERE(sphere));

    let mut total = 0;
    let _ = geom.attach(ceiling, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(floor, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(back, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(left, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(right, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(short_block_top, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(short_block_bot, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(short_block_right, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(short_block_left, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(short_block_back, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(short_block_front, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(tall_block_top, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(tall_block_bot, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(tall_block_right, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(tall_block_left, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(tall_block_back, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(tall_block_front, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(mirror, &device, &mut scene)?;
    total += 1;
    let _ = geom.attach(sphere_geometry, &device, &mut scene)?;
    total += 1;

    let ambient = Light {
        light_type: LightType::Ambient,
        color: LinearRgba::rgb(0.07, 0.07, 0.07), // color: LinearRgba::rgb(1.0, 1.0, 1.0)
    };
    lights.lights.push(ambient);

    // let n_points_dim = 3; // must be 1, 3 or 5
    // let n_half: i32 = (n_points_dim - 1) / 2;
    // for x in -n_half..(n_half + 1) {
    //     for z in -n_half..(n_half + 1) {
    //         let power = 1.0 / ((n_points_dim * n_points_dim) as f32);
    //         let point_light = Light {
    //             light_type: LightType::Point(Vec3::new(
    //                 278.0 + (x as f32 * 100.0),
    //                 545.0,
    //                 280.0 + (z as f32 * 100.0),
    //             )),
    //             color: LinearRgba::rgb(power, power, power),
    //         };
    //         lights.lights.push(point_light);
    //     }
    // }

    let size = 50.0;
    for i in -1..2 {
        let area_square = Light {
            color: LinearRgba::rgb(1.2, 1.2, 1.2),
            light_type: LightType::AreaQuad(LightQuad {
                bottom_left: Vec3::new(250.0 + (i * 250) as f32, 545.0, 250.0 + (i * 250) as f32),
                u_vec: Vec3::X * size,
                v_vec: Vec3::Z * size,
            }),
        };
        lights.lights.push(area_square);
    }

    Ok(total)
}

// WARNING: adds meshes one by one. ignores children. assumes all primitives are triangles
pub fn add_gltf(
    geom: &mut GeomStorage,
    device: &Device,
    mut scene: &mut Scene<'_>,
    gltf_doc: &gltf::Document,
    gltf_buff: &Vec<gltf::buffer::Data>,
    transform: &Transform,
    material: Material,
) -> Result<u32> {
    let mut total = 0;

    let matrix = transform.compute_matrix();

    // for scene in gltf.scenes() {
    //     for node in scene.nodes() {
    //         if let Some(mesh) = node.mesh() {
    //             for primitive in mesh.primitives() {
    //             }
    //         }
    //     }
    // }

    for mesh in gltf_doc.meshes() {
        for primitive in mesh.primitives() {
            let mut verts: Vec<(f32, f32, f32)> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();

            let reader = primitive.reader(|buffer| Some(&gltf_buff[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    let pos = Vec4::new(
                        vertex_position[0],
                        vertex_position[1],
                        vertex_position[2],
                        1.0,
                    );
                    let transformed = matrix * pos;
                    verts.push((transformed.x, transformed.y, transformed.z));
                }
            }
            if let Some(iter) = reader.read_indices() {
                match iter {
                    ReadIndices::U8(inner) => {
                        for index in inner {
                            indices.push(index as u32);
                        }
                    }
                    ReadIndices::U16(inner) => {
                        for index in inner {
                            indices.push(index as u32);
                        }
                    }
                    ReadIndices::U32(inner) => {
                        for index in inner {
                            indices.push(index);
                        }
                    }
                }
            }

            // panic if not divisible by 3?????
            let triangle_indices: Vec<(u32, u32, u32)> = indices
                .chunks(3)
                // .filter(|chunk| chunk.len() == 3) // Only keep complete groups of 3
                .map(|chunk| (chunk[0], chunk[1], chunk[2]))
                .collect();

            let new_mesh = Mesh {
                verts,
                indices: triangle_indices,
            };
            let geometry = Geometry::with_material(material.clone(), GeomInfo::MESH(new_mesh));
            let _ = geom.attach(geometry, &device, &mut scene)?;
            total += 1;
        }
    }

    Ok(total)
}

// 2122
// 3126
