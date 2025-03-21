use bevy_color::LinearRgba;
use bevy_math::*;

mod storage;
pub use storage::*;

mod material;
pub use material::*;

#[derive(Clone)]
pub struct SphereGeometry {
    pub radius: f32,
    pub center: Vec3,
}

#[derive(Clone, Default)]
pub struct MeshGeometry {
    pub verts: Vec<(f32, f32, f32)>,
    pub indices: Vec<(u32, u32, u32)>,
    pub tex_coords: Vec<Vec2>, // not sent to embree
}

impl MeshGeometry {
    pub fn transform(&mut self, matrix: Mat4) {
        for vert in &mut self.verts {
            let pos = Vec4::new(vert.0, vert.1, vert.2, 1.0);
            let new_pos = matrix * pos;
            vert.0 = new_pos.x;
            vert.1 = new_pos.y;
            vert.2 = new_pos.z;
        }
    }
}

#[derive(Clone)]
pub enum GeomInfo {
    Mesh(MeshGeometry),
    Sphere(SphereGeometry),
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
            GeomInfo::Mesh(ref mesh) => {
                let w = 1.0 - u - v;

                // get the indices for this triangle
                let (i0, i1, i2) = mesh.indices[prim_id as usize];

                // // position of the vertices
                // let p0 = mesh.verts[i0];
                // let p1 = mesh.verts[i1];
                // let p2 = mesh.verts[i2];

                // uv of the vertices
                let vertex_uv_0 = mesh.tex_coords[i0 as usize];
                let vertex_uv_1 = mesh.tex_coords[i1 as usize];
                let vertex_uv_2 = mesh.tex_coords[i2 as usize];

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

    pub fn transform(&mut self, matrix: Mat4) {
        match self.info {
            GeomInfo::Mesh(ref mut mesh) => {
                mesh.transform(matrix);
            }
            _ => {
                panic!("transform for spheres not implemented yet");
            }
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
