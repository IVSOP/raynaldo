use std::collections::HashMap;

use bevy_color::LinearRgba;
use glam::*;
use crate::common::*;
use embree4_rs::{geometry::TriangleMeshGeometry, Device, Scene};
use anyhow::Result;

pub struct Mesh {
    pub verts: Vec<(f32, f32, f32)>,
    pub indices: Vec<(u32, u32, u32)>,
    pub material: Material,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub color: LinearRgba,
}

impl Material {
    pub fn color(color: LinearRgba) -> Self {
        Self {
            color,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: LinearRgba::RED,
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
    pub fn attach<'a>(&mut self, mesh: Mesh, device: &Device, scene: &mut Scene<'a>) -> Result<u32> {
        let embree_mesh = TriangleMeshGeometry::try_new(device, &mesh.verts, &mesh.indices)?;
        let id: u32 = scene.attach_geometry(&embree_mesh)?;

        self.meshes.insert(id, mesh);
        Ok(id)
    }

    pub fn get(&self, id: u32) -> Option<&Mesh> {
        return self.meshes.get(&id)
    }
}
