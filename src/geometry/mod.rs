use std::collections::HashMap;

// use crate::common::*;
use anyhow::Result;
use bevy_color::LinearRgba;
use embree4_rs::{
    Device, Scene,
    geometry::{SphereGeometry, TriangleMeshGeometry},
};
use glam::*;

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
    // pub transparency: f32,
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
            // transparency: 0.0,
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

pub struct GeomStorage {
    pub geom: HashMap<u32, Geometry>,
}

impl Default for GeomStorage {
    fn default() -> Self {
        Self {
            geom: HashMap::new(),
        }
    }
}

impl GeomStorage {
    // returns the ID of this mesh, or error
    // the mesh is moved into internal structure
    pub fn attach(
        &mut self,
        geom: Geometry,
        device: &Device,
        scene: &mut Scene<'_>,
    ) -> Result<u32> {
        let id: u32;

        match geom.info {
            GeomInfo::MESH(ref mesh) => {
                let embree_mesh =
                    TriangleMeshGeometry::try_new(device, &mesh.verts, &mesh.indices)?;
                id = scene.attach_geometry(&embree_mesh)?;
            }
            GeomInfo::SPHERE(ref sphere) => {
                let embree_geom = SphereGeometry::try_new(
                    device,
                    (sphere.center.x, sphere.center.y, sphere.center.z),
                    sphere.radius,
                )?;
                id = scene.attach_geometry(&embree_geom)?;
            }
        }
        self.geom.insert(id, geom);
        Ok(id)
    }

    pub fn get(&self, id: u32) -> Option<&Geometry> {
        return self.geom.get(&id);
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
