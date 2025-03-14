use std::collections::HashMap;

use crate::common::*;
use anyhow::Result;
use bevy_color::LinearRgba;
use embree4_rs::{Device, Scene, geometry::TriangleMeshGeometry};
use glam::*;

pub struct Mesh {
    pub verts: Vec<(f32, f32, f32)>,
    pub indices: Vec<(u32, u32, u32)>,
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
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: LinearRgba::RED,
            diffuse: LinearRgba::RED,
            specular: LinearRgba::RED,
            transmission: LinearRgba::RED,
            refraction: 1.0,
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            verts: Vec::new(),
            indices: Vec::new(),
            material: Material::default(),
        }
    }
}

impl Mesh {
    // number of triangles, NOT vertices
    pub fn _new_tri_capacity(num_triangles: usize) -> Self {
        Self {
            verts: Vec::with_capacity(3 * num_triangles),
            indices: Vec::with_capacity(num_triangles),
            ..default()
        }
    }

    // pub fn new() -> Self {
    //     Self {
    //         ..default()
    //     }
    // }

    pub fn with_material(material: Material) -> Self {
        Self {
            material,
            ..default()
        }
    }
}

pub struct MeshStorage {
    pub meshes: HashMap<u32, Mesh>,
}

impl Default for MeshStorage {
    fn default() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }
}

impl MeshStorage {
    // returns the ID of this mesh, or error
    // the mesh is moved into internal structure
    pub fn attach(&mut self, mesh: Mesh, device: &Device, scene: &mut Scene<'_>) -> Result<u32> {
        let embree_mesh = TriangleMeshGeometry::try_new(device, &mesh.verts, &mesh.indices)?;
        let id: u32 = scene.attach_geometry(&embree_mesh)?;

        self.meshes.insert(id, mesh);
        Ok(id)
    }

    pub fn get(&self, id: u32) -> Option<&Mesh> {
        return self.meshes.get(&id);
    }
}

pub struct Light {
    pub light_type: LightType,
    pub color: LinearRgba,
}

pub enum LightType {
    AMBIENT,
    POINT(Vec3), // stores position
                 // AREA,
}

pub struct LightStorage {
    pub lights: Vec<Light>,
}

impl Default for LightStorage {
    fn default() -> Self {
        Self { lights: Vec::new() }
    }
}
