// use crate::common::*;
use anyhow::Result;
use bevy_color::LinearRgba;
use bevy_math::*;
use embree4_rs::{
    Device, Scene,
    geometry::{SphereGeometry, TriangleMeshGeometry},
};

mod storage;
pub use storage::*;

#[derive(Clone)]
pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
}

#[derive(Clone)]
pub struct Mesh {
    pub verts: Vec<(f32, f32, f32)>,
    pub indices: Vec<(u32, u32, u32)>,
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

#[derive(Debug, Clone)]
pub struct Material {
    pub color: LinearRgba,
    // pub emissive: LinearRgba,
    pub diffuse: LinearRgba,
    pub specular: LinearRgba,
    pub transmission: LinearRgba,
    pub refraction: f32,
    pub reflectivity: f32,
    pub transparency: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: LinearRgba::RED,
            diffuse: LinearRgba::RED,
            specular: LinearRgba::RED,
            transmission: LinearRgba::RED,
            refraction: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            verts: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl Geometry {
    pub fn with_material(material: Material, info: GeomInfo) -> Self {
        Self { material, info }
    }
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
