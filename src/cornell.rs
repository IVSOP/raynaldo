use crate::color::Rgba;
use crate::geometry::*;
use GeomInfo::Mesh;
use anyhow::*;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use gltf::{
    // Gltf,
    // buffer::Data,
    mesh::util::*,
};

pub fn cornell_box(store: &mut Scene) -> Result<()> {
    let mut ceiling_mesh = MeshGeometry::default();
    ceiling_mesh.verts.push((556.0, 548.8, 0.0));
    ceiling_mesh.verts.push((0.0, 548.8, 0.0));
    ceiling_mesh.verts.push((0.0, 548.8, 559.2));
    ceiling_mesh.verts.push((556.0, 548.8, 559.2));
    ceiling_mesh.indices.push((0, 2, 1));
    ceiling_mesh.indices.push((0, 3, 2));
    let ceiling = Geometry::with_material(Material::WHITE_MATERIAL, Mesh(ceiling_mesh));

    let mut floor_mesh = MeshGeometry::default();
    floor_mesh.verts.push((552.8, 0.0, 0.0));
    floor_mesh.verts.push((0.0, 0.0, 0.0));
    floor_mesh.verts.push((0.0, 0.0, 559.2));
    floor_mesh.verts.push((549.6, 0.0, 559.2));
    floor_mesh.indices.push((0, 1, 2));
    floor_mesh.indices.push((0, 2, 3));
    let floor = Geometry::with_material(Material::WHITE_MATERIAL, Mesh(floor_mesh));

    let mut back_mesh = MeshGeometry::default();
    back_mesh.verts.push((0.0, 0.0, 559.2));
    back_mesh.verts.push((549.6, 0.0, 559.2));
    back_mesh.verts.push((556.0, 548.8, 559.2));
    back_mesh.verts.push((0.0, 548.8, 559.2));
    back_mesh.indices.push((2, 1, 0));
    back_mesh.indices.push((3, 2, 0));
    let back = Geometry::with_material(Material::WHITE_MATERIAL, Mesh(back_mesh));

    let mut left_mesh = MeshGeometry::default();
    left_mesh.verts.push((0.0, 0.0, 0.0));
    left_mesh.verts.push((0., 0., 559.2));
    left_mesh.verts.push((0., 548.8, 559.2));
    left_mesh.verts.push((0., 548.8, 0.));

    left_mesh.tex_coords.push(Vec2::new(1.0, 0.0));
    left_mesh.tex_coords.push(Vec2::new(0.0, 0.0));
    left_mesh.tex_coords.push(Vec2::new(0.0, 1.0));
    left_mesh.tex_coords.push(Vec2::new(1.0, 1.0));

    left_mesh.indices.push((0, 2, 1));
    left_mesh.indices.push((0, 3, 2));
    let left = Geometry::with_material(Material::UV_MATERIAL, Mesh(left_mesh));

    let mut right_mesh = MeshGeometry::default();
    right_mesh.verts.push((552.8, 0.0, 0.));
    right_mesh.verts.push((549.6, 0., 559.2));
    right_mesh.verts.push((549.6, 548.8, 559.2));
    right_mesh.verts.push((552.8, 548.8, 0.));
    right_mesh.indices.push((0, 1, 2));
    right_mesh.indices.push((0, 2, 3));
    let right = Geometry::with_material(Material::RED_MATERIAL, Mesh(right_mesh));

    let mut short_block_top_mesh = MeshGeometry::default();
    short_block_top_mesh.verts.push((130.0, 165.0, 65.0));
    short_block_top_mesh.verts.push((82.0, 165.0, 225.0));
    short_block_top_mesh.verts.push((240.0, 165.0, 272.0));
    short_block_top_mesh.verts.push((290.0, 165.0, 114.0));
    short_block_top_mesh.indices.push((0, 1, 2));
    short_block_top_mesh.indices.push((0, 2, 3));
    let short_block_top =
        Geometry::with_material(Material::ORANGE_MATERIAL, Mesh(short_block_top_mesh));

    let mut short_block_bot_mesh = MeshGeometry::default();
    short_block_bot_mesh.verts.push((130.0, 0.01, 65.0));
    short_block_bot_mesh.verts.push((82.0, 0.01, 225.0));
    short_block_bot_mesh.verts.push((240.0, 0.01, 272.0));
    short_block_bot_mesh.verts.push((290.0, 0.01, 114.0));
    short_block_bot_mesh.indices.push((0, 1, 2));
    short_block_bot_mesh.indices.push((0, 2, 3));
    let short_block_bot =
        Geometry::with_material(Material::ORANGE_MATERIAL, Mesh(short_block_bot_mesh));

    let mut short_block_left_mesh = MeshGeometry::default();
    short_block_left_mesh.verts.push((290.0, 0.0, 114.0));
    short_block_left_mesh.verts.push((290.0, 165.0, 114.0));
    short_block_left_mesh.verts.push((240.0, 165.0, 272.0));
    short_block_left_mesh.verts.push((240.0, 0.0, 272.0));
    short_block_left_mesh.indices.push((0, 1, 2));
    short_block_left_mesh.indices.push((0, 2, 3));
    let short_block_left =
        Geometry::with_material(Material::ORANGE_MATERIAL, Mesh(short_block_left_mesh));

    let mut short_block_back_mesh = MeshGeometry::default();
    short_block_back_mesh.verts.push((240.0, 0.0, 272.0));
    short_block_back_mesh.verts.push((240.0, 165.0, 272.0));
    short_block_back_mesh.verts.push((82.0, 165., 225.0));
    short_block_back_mesh.verts.push((82.0, 0.0, 225.0));
    short_block_back_mesh.indices.push((0, 1, 2));
    short_block_back_mesh.indices.push((0, 2, 3));
    let short_block_back =
        Geometry::with_material(Material::ORANGE_MATERIAL, Mesh(short_block_back_mesh));

    let mut short_block_right_mesh = MeshGeometry::default();
    short_block_right_mesh.verts.push((82.0, 0.0, 225.0));
    short_block_right_mesh.verts.push((82.0, 165.0, 225.0));
    short_block_right_mesh.verts.push((130.0, 165.0, 65.0));
    short_block_right_mesh.verts.push((130.0, 0.0, 65.0));
    short_block_right_mesh.indices.push((0, 1, 2));
    short_block_right_mesh.indices.push((0, 2, 3));
    let short_block_right =
        Geometry::with_material(Material::ORANGE_MATERIAL, Mesh(short_block_right_mesh));

    let mut short_block_front_mesh = MeshGeometry::default();
    short_block_front_mesh.verts.push((130.0, 0.0, 65.0));
    short_block_front_mesh.verts.push((130.0, 165.0, 65.0));
    short_block_front_mesh.verts.push((290.0, 165.0, 114.0));
    short_block_front_mesh.verts.push((290.0, 0.0, 114.0));
    short_block_front_mesh.indices.push((0, 1, 2));
    short_block_front_mesh.indices.push((0, 2, 3));
    let short_block_front =
        Geometry::with_material(Material::ORANGE_MATERIAL, Mesh(short_block_front_mesh));

    let mut tall_block_top_mesh = MeshGeometry::default();
    tall_block_top_mesh.verts.push((423.0, 330.0, 247.0));
    tall_block_top_mesh.verts.push((265.0, 330.0, 296.0));
    tall_block_top_mesh.verts.push((314.0, 330.0, 456.0));
    tall_block_top_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_top_mesh.indices.push((0, 1, 2));
    tall_block_top_mesh.indices.push((0, 2, 3));
    let tall_block_top =
        Geometry::with_material(Material::BLUE_MATERIAL, Mesh(tall_block_top_mesh));

    let mut tall_block_bot_mesh = MeshGeometry::default();
    tall_block_bot_mesh.verts.push((423.0, 0.1, 247.0));
    tall_block_bot_mesh.verts.push((265.0, 0.1, 296.0));
    tall_block_bot_mesh.verts.push((314.0, 0.1, 456.0));
    tall_block_bot_mesh.verts.push((472.0, 0.1, 406.0));
    tall_block_bot_mesh.indices.push((0, 1, 2));
    tall_block_bot_mesh.indices.push((0, 2, 3));
    let tall_block_bot =
        Geometry::with_material(Material::BLUE_MATERIAL, Mesh(tall_block_bot_mesh));

    let mut tall_block_left_mesh = MeshGeometry::default();
    tall_block_left_mesh.verts.push((423.0, 0.0, 247.0));
    tall_block_left_mesh.verts.push((423.0, 330.0, 247.0));
    tall_block_left_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_left_mesh.verts.push((472.0, 0.0, 406.0));
    tall_block_left_mesh.indices.push((0, 1, 2));
    tall_block_left_mesh.indices.push((0, 2, 3));
    let tall_block_left =
        Geometry::with_material(Material::BLUE_MATERIAL, Mesh(tall_block_left_mesh));

    let mut tall_block_back_mesh = MeshGeometry::default();
    tall_block_back_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_back_mesh.verts.push((472.0, 330.0, 406.0));
    tall_block_back_mesh.verts.push((314.0, 330.0, 456.0));
    tall_block_back_mesh.verts.push((314.0, 0.0, 406.0));
    tall_block_back_mesh.indices.push((0, 1, 2));
    tall_block_back_mesh.indices.push((0, 2, 3));
    let tall_block_back =
        Geometry::with_material(Material::BLUE_MATERIAL, Mesh(tall_block_back_mesh));

    let mut tall_block_right_mesh = MeshGeometry::default();
    tall_block_right_mesh.verts.push((314.0, 0.0, 456.0));
    tall_block_right_mesh.verts.push((314.0, 330.0, 456.0));
    tall_block_right_mesh.verts.push((265.0, 330.0, 296.0));
    tall_block_right_mesh.verts.push((265.0, 0.0, 296.0));
    tall_block_right_mesh.indices.push((0, 1, 2));
    tall_block_right_mesh.indices.push((0, 2, 3));
    let tall_block_right =
        Geometry::with_material(Material::BLUE_MATERIAL, Mesh(tall_block_right_mesh));

    let mut tall_block_front_mesh = MeshGeometry::default();
    tall_block_front_mesh.verts.push((265.0, 0.0, 296.0));
    tall_block_front_mesh.verts.push((265.0, 330.0, 296.0));
    tall_block_front_mesh.verts.push((423.0, 330.0, 247.0));
    tall_block_front_mesh.verts.push((423.0, 0.0, 247.0));
    tall_block_front_mesh.indices.push((0, 1, 2));
    tall_block_front_mesh.indices.push((0, 2, 3));
    let tall_block_front =
        Geometry::with_material(Material::BLUE_MATERIAL, Mesh(tall_block_front_mesh));

    let mut mirror_mesh = MeshGeometry::default();
    mirror_mesh.verts.push((552.0, 50.0, 50.));
    mirror_mesh.verts.push((549.0, 50.0, 509.2));
    mirror_mesh.verts.push((549.0, 488.8, 509.2));
    mirror_mesh.verts.push((552.0, 488.8, 50.0));
    mirror_mesh.indices.push((0, 1, 2));
    mirror_mesh.indices.push((0, 2, 3));
    let mirror = Geometry::with_material(Material::MIRROR_MATERIAL, Mesh(mirror_mesh));

    let sphere = SphereGeometry {
        radius: 110.0,
        center: Vec3::new(160.0, 320.0, 225.0),
    };
    let sphere_geometry =
        Geometry::with_material(Material::GLASS_MATERIAL, GeomInfo::Sphere(sphere));

    store.add_geometry(ceiling);
    store.add_geometry(floor);
    store.add_geometry(back);
    store.add_geometry(left);
    store.add_geometry(right);
    store.add_geometry(short_block_top);
    store.add_geometry(short_block_bot);
    store.add_geometry(short_block_right);
    store.add_geometry(short_block_left);
    store.add_geometry(short_block_back);
    store.add_geometry(short_block_front);
    store.add_geometry(tall_block_top);
    store.add_geometry(tall_block_bot);
    store.add_geometry(tall_block_right);
    store.add_geometry(tall_block_left);
    store.add_geometry(tall_block_back);
    store.add_geometry(tall_block_front);
    store.add_geometry(mirror);
    store.add_geometry(sphere_geometry);

    let (cube_gltf_doc, cube_gltf_buff, _) = gltf::import("assets/cube.glb")?;
    let cube_mesh = get_gltf_meshes(&cube_gltf_doc, &cube_gltf_buff)[0].clone();

    let transform = glam::Mat4::from_scale_rotation_translation(
        Vec3::splat(50.0),
        Quat::default(),
        Vec3::new(350.0, 50.0, 75.0),
    );
    let mut bright_red_cube = cube_mesh.clone();
    bright_red_cube.transform(transform);
    store.add_geometry(Geometry::with_material(
        Material::EMISSIVE_MATERIAL,
        Mesh(bright_red_cube),
    ));

    // let ambient = Light {
    //     light_type: LightType::Ambient,
    //     color: Rgba::rgb(0.07, 0.07, 0.07), // color: LinearRgba::rgb(1.0, 1.0, 1.0)
    // };
    // store.lights.push(ambient);

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
            color: Rgba::rgb(250000.0, 250000.0, 250000.0),
            light_type: LightType::AreaQuad(LightQuad {
                bottom_left: Vec3::new(250.0 + (i * 250) as f32, 545.0, 250.0 + (i * 250) as f32),
                u_vec: Vec3::X * size,
                v_vec: Vec3::Z * size,
                normal: Vec3::NEG_Y,
            }),
        };
        store.lights.push(area_square);
    }

    Ok(())
}

// WARN: adds meshes one by one. ignores children. assumes all primitives are triangles
pub fn get_gltf_meshes(
    gltf_doc: &gltf::Document,
    gltf_buff: &Vec<gltf::buffer::Data>,
    // transform: &Transform,
) -> Vec<MeshGeometry> {
    // let matrix = transform.compute_matrix();

    // for scene in gltf.scenes() {
    //     for node in scene.nodes() {
    //         if let Some(mesh) = node.mesh() {
    //             for primitive in mesh.primitives() {
    //             }
    //         }
    //     }
    // }

    let meshes_iter = gltf_doc.meshes();

    let mut meshes: Vec<MeshGeometry> = Vec::with_capacity(meshes_iter.len());

    for mesh in meshes_iter {
        for primitive in mesh.primitives() {
            let mut verts: Vec<(f32, f32, f32)> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();
            let mut tex_coords: Vec<Vec2> = Vec::new();

            let reader = primitive.reader(|buffer| Some(&gltf_buff[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    // let pos = Vec4::new(
                    //     vertex_position[0],
                    //     vertex_position[1],
                    //     vertex_position[2],
                    //     1.0,
                    // );
                    // let transformed = matrix * pos;
                    // verts.push((transformed.x, transformed.y, transformed.z));
                    verts.push((vertex_position[0], vertex_position[1], vertex_position[2]));
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

            if let Some(iter) = reader.read_tex_coords(0) {
                match iter {
                    ReadTexCoords::U8(inner) => {
                        for uv in inner {
                            tex_coords.push(Vec2::new(uv[0] as f32, uv[1] as f32));
                        }
                    }
                    ReadTexCoords::U16(inner) => {
                        for uv in inner {
                            tex_coords.push(Vec2::new(uv[0] as f32, uv[1] as f32));
                        }
                    }
                    ReadTexCoords::F32(inner) => {
                        for uv in inner {
                            tex_coords.push(Vec2::new(uv[0], uv[1]));
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

            let new_mesh = MeshGeometry {
                verts,
                indices: triangle_indices,
                tex_coords,
            };
            meshes.push(new_mesh);
        }
    }

    meshes
}

pub fn add_gltf(
    store: &mut Scene,
    gltf_doc: &gltf::Document,
    gltf_buff: &Vec<gltf::buffer::Data>,
    matrix: Mat4,
    material: &Material,
) -> Result<()> {
    for mut mesh in get_gltf_meshes(gltf_doc, gltf_buff) {
        mesh.transform(matrix);
        let geometry = Geometry::with_material(material.clone(), Mesh(mesh));
        let _ = store.add_geometry(geometry);
    }

    Ok(())
}

pub fn add_skybox(store: &mut Scene) -> Result<()> {
    let mut front = MeshGeometry::default();
    let mut back = MeshGeometry::default();
    let mut right = MeshGeometry::default();
    let mut left = MeshGeometry::default();
    let mut top = MeshGeometry::default();
    let mut bottom = MeshGeometry::default();

    let mut front_material = Material::CUBEMAP_MATERIAL;
    front_material.emissive = Texture::Image(1);
    let mut back_material = Material::CUBEMAP_MATERIAL;
    back_material.emissive = Texture::Image(2);
    let mut right_material = Material::CUBEMAP_MATERIAL;
    right_material.emissive = Texture::Image(3);
    let mut left_material = Material::CUBEMAP_MATERIAL;
    left_material.emissive = Texture::Image(4);
    let mut top_material = Material::CUBEMAP_MATERIAL;
    top_material.emissive = Texture::Image(5);
    let mut bottom_material = Material::CUBEMAP_MATERIAL;
    bottom_material.emissive = Texture::Image(6);

    let transform = Mat4::from_scale_rotation_translation(
        Vec3::splat(10000.0),
        Default::default(),
        Default::default(),
    );

    let bottom_left_front = transform * Vec4::new(-0.5, -0.5, 0.5, 1.0);
    let top_left_front = transform * Vec4::new(-0.5, 0.5, 0.5, 1.0);
    let top_right_front = transform * Vec4::new(0.5, 0.5, 0.5, 1.0);
    let bottom_right_front = transform * Vec4::new(0.5, -0.5, 0.5, 1.0);
    let bottom_left_back = transform * Vec4::new(-0.5, -0.5, -0.5, 1.0);
    let top_left_back = transform * Vec4::new(-0.5, 0.5, -0.5, 1.0);
    let top_right_back = transform * Vec4::new(0.5, 0.5, -0.5, 1.0);
    let bottom_right_back = transform * Vec4::new(0.5, -0.5, -0.5, 1.0);

    front.indices.push((0, 1, 2));
    front.indices.push((0, 2, 3));
    back.indices.push((0, 1, 2));
    back.indices.push((0, 2, 3));
    right.indices.push((0, 1, 2));
    right.indices.push((0, 2, 3));
    left.indices.push((0, 1, 2));
    left.indices.push((0, 2, 3));
    top.indices.push((0, 1, 2));
    top.indices.push((0, 2, 3));
    bottom.indices.push((0, 1, 2));
    bottom.indices.push((0, 2, 3));

    front.verts.push((
        bottom_left_front.x,
        bottom_left_front.y,
        bottom_left_front.z,
    ));
    front.verts.push((
        bottom_right_front.x,
        bottom_right_front.y,
        bottom_right_front.z,
    ));
    front
        .verts
        .push((top_right_front.x, top_right_front.y, top_right_front.z));
    front
        .verts
        .push((top_left_front.x, top_left_front.y, top_left_front.z));
    front.tex_coords.push(Vec2::new(0.0, 0.0));
    front.tex_coords.push(Vec2::new(1.0, 0.0));
    front.tex_coords.push(Vec2::new(1.0, 1.0));
    front.tex_coords.push(Vec2::new(0.0, 1.0));

    back.verts.push((
        bottom_right_back.x,
        bottom_right_back.y,
        bottom_right_back.z,
    ));
    back.verts
        .push((bottom_left_back.x, bottom_left_back.y, bottom_left_back.z));
    back.verts
        .push((top_left_back.x, top_left_back.y, top_left_back.z));
    back.verts
        .push((top_right_back.x, top_right_back.y, top_right_back.z));
    back.tex_coords.push(Vec2::new(0.0, 0.0));
    back.tex_coords.push(Vec2::new(1.0, 0.0));
    back.tex_coords.push(Vec2::new(1.0, 1.0));
    back.tex_coords.push(Vec2::new(0.0, 1.0));

    right.verts.push((
        bottom_right_front.x,
        bottom_right_front.y,
        bottom_right_front.z,
    ));
    right.verts.push((
        bottom_right_back.x,
        bottom_right_back.y,
        bottom_right_back.z,
    ));
    right
        .verts
        .push((top_right_back.x, top_right_back.y, top_right_back.z));
    right
        .verts
        .push((top_right_front.x, top_right_front.y, top_right_front.z));
    right.tex_coords.push(Vec2::new(0.0, 0.0));
    right.tex_coords.push(Vec2::new(1.0, 0.0));
    right.tex_coords.push(Vec2::new(1.0, 1.0));
    right.tex_coords.push(Vec2::new(0.0, 1.0));

    left.verts
        .push((bottom_left_back.x, bottom_left_back.y, bottom_left_back.z));
    left.verts.push((
        bottom_left_front.x,
        bottom_left_front.y,
        bottom_left_front.z,
    ));
    left.verts
        .push((top_left_front.x, top_left_front.y, top_left_front.z));
    left.verts
        .push((top_left_back.x, top_left_back.y, top_left_back.z));
    left.tex_coords.push(Vec2::new(0.0, 0.0));
    left.tex_coords.push(Vec2::new(1.0, 0.0));
    left.tex_coords.push(Vec2::new(1.0, 1.0));
    left.tex_coords.push(Vec2::new(0.0, 1.0));

    top.verts
        .push((top_left_front.x, top_left_front.y, top_left_front.z));
    top.verts
        .push((top_right_front.x, top_right_front.y, top_right_front.z));
    top.verts
        .push((top_right_back.x, top_right_back.y, top_right_back.z));
    top.verts
        .push((top_left_back.x, top_left_back.y, top_left_back.z));
    top.tex_coords.push(Vec2::new(0.0, 0.0));
    top.tex_coords.push(Vec2::new(1.0, 0.0));
    top.tex_coords.push(Vec2::new(1.0, 1.0));
    top.tex_coords.push(Vec2::new(0.0, 1.0));

    bottom
        .verts
        .push((bottom_left_back.x, bottom_left_back.y, bottom_left_back.z));
    bottom.verts.push((
        bottom_right_back.x,
        bottom_right_back.y,
        bottom_right_back.z,
    ));
    bottom.verts.push((
        bottom_right_front.x,
        bottom_right_front.y,
        bottom_right_front.z,
    ));
    bottom.verts.push((
        bottom_left_front.x,
        bottom_left_front.y,
        bottom_left_front.z,
    ));
    bottom.tex_coords.push(Vec2::new(0.0, 0.0));
    bottom.tex_coords.push(Vec2::new(1.0, 0.0));
    bottom.tex_coords.push(Vec2::new(1.0, 1.0));
    bottom.tex_coords.push(Vec2::new(0.0, 1.0));

    store.add_geometry(Geometry::with_material(front_material, Mesh(front)));
    store.add_geometry(Geometry::with_material(back_material, Mesh(back)));
    store.add_geometry(Geometry::with_material(right_material, Mesh(right)));
    store.add_geometry(Geometry::with_material(left_material, Mesh(left)));
    store.add_geometry(Geometry::with_material(top_material, Mesh(top)));
    store.add_geometry(Geometry::with_material(bottom_material, Mesh(bottom)));

    Ok(())
}
