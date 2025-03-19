// use crate::common::*;
use anyhow::Result;
use bevy_color::LinearRgba;
use bevy_math::*;
use embree4_rs::{
    Device, Scene,
    geometry::{SphereGeometry, TriangleMeshGeometry},
};
use image::{Rgba32FImage, imageops::sample_bilinear};

mod storage;
pub use storage::*;

mod material;
pub use material::*;

#[derive(Clone)]
pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
}

#[derive(Clone)]
pub struct Mesh {
    pub verts: Vec<(f32, f32, f32)>,
    pub indices: Vec<(u32, u32, u32)>,
    pub tex_coords: Vec<Vec2>, // not sent to embree
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            verts: Vec::new(),
            indices: Vec::new(),
            tex_coords: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub enum GeomInfo {
    MESH(Mesh),
    SPHERE(Sphere),
}

#[derive(Clone)]
pub struct Geometry {
    pub info: GeomInfo,
    pub material: Material,
}

impl Geometry {
    pub fn with_material(material: Material, info: GeomInfo) -> Self {
        Self { material, info }
    }

    // u v are actually the uv passed in by embree
    pub fn compute_uv(&self, u: f32, v: f32, prim_id: u32) -> Vec2 {
        match self.info {
            GeomInfo::MESH(ref mesh) => {
                let w = 1.0 - u - v;
                let id = prim_id as usize;

                // get the indices for this triangle
                let indices = mesh.indices[id];
                let i0 = indices.0 as usize;
                let i1 = indices.1 as usize;
                let i2 = indices.2 as usize;

                // // position of the vertices
                // let p0 = mesh.verts[i0];
                // let p1 = mesh.verts[i1];
                // let p2 = mesh.verts[i2];

                // uv of the vertices
                let vertex_uv_0 = mesh.tex_coords[i0];
                let vertex_uv_1 = mesh.tex_coords[i1];
                let vertex_uv_2 = mesh.tex_coords[i2];

                let actual_u =
                    (vertex_uv_0.x * w + vertex_uv_1.x * u + vertex_uv_2.x * v).clamp(0.0, 1.0);
                // FIX: textures are flipped vertically. is it this math or the image loader?
                // for now I just added 1.0 - ... here
                let actual_v = 1.0
                    - (vertex_uv_0.y * w + vertex_uv_1.y * u + vertex_uv_2.y * v).clamp(0.0, 1.0);

                Vec2::new(actual_u, actual_v)
            }
            _ => Vec2::ZERO, // TODO: how do I implement this
        }
    }
}

#[derive(Debug, Clone)]
pub enum Texture {
    Solid(LinearRgba),
    Image(u32), // an id
}

pub struct Light {
    pub light_type: LightType,
    pub color: LinearRgba,
}

pub enum LightType {
    Ambient,
    Point(Vec3), // stores position
    AreaQuad(LightQuad),
}

pub struct LightQuad {
    pub bottom_left: Vec3,
    pub u_vec: Vec3, // direction travelled when u varies, multiplied by size of each side. bottom_left + u_vec == bottom_right
    pub v_vec: Vec3,
}
