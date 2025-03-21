use crate::geometry::{GeomInfo, Geometry, Light, Texture};
use anyhow::{Context, Result};
use bevy_color::LinearRgba;
use bevy_math::Vec2;
use embree4_rs::geometry::SphereGeometry;
use image::ImageReader;
use image::Rgba32FImage;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct SceneStorage {
    pub lights: Vec<Light>,
    pub geom: HashMap<u32, Geometry>,
    pub textures: Vec<Rgba32FImage>,
}

impl SceneStorage {
    pub fn attach_geometry(
        &mut self,
        geom: Geometry,
        device: &embree4_rs::Device,
        scene: &mut embree4_rs::Scene<'_>,
    ) -> Result<u32> {
        let id = match geom.info {
            GeomInfo::Mesh(ref mesh) => {
                let embree_mesh = embree4_rs::geometry::TriangleMeshGeometry::try_new(
                    device,
                    &mesh.verts,
                    &mesh.indices,
                )?;
                scene.attach_geometry(&embree_mesh)?
            }
            GeomInfo::Sphere(ref sphere) => {
                let embree_geom = SphereGeometry::try_new(
                    device,
                    (sphere.center.x, sphere.center.y, sphere.center.z),
                    sphere.radius,
                )?;
                scene.attach_geometry(&embree_geom)?
            }
        };
        self.geom.insert(id, geom);
        Ok(id)
    }

    pub fn get_geometry(&self, id: u32) -> Option<&Geometry> {
        self.geom.get(&id)
    }

    // use this instead of default so that the default texture is loaded
    pub fn new() -> Result<Self> {
        let mut res = Self {
            lights: Vec::new(),
            geom: HashMap::new(),
            textures: Vec::new(),
        };

        let default_texture: Rgba32FImage = ImageReader::open("assets/textures/uv.png")
            .context("Default texture (assets/textures/uv.png) does not exist")?
            .decode()
            .context("Error decoding texture assets/textures/uv.png")?
            .into_rgba32f();

        res.textures.push(default_texture);

        Ok(res)
    }

    // I assume opening the file is very fast but decoding() and into_rgba32f() are slow, so I just made the whole thing parallel
    // cursed, does not error out instantly
    pub fn load_textures_batch(&mut self, paths: &[&str]) -> Result<()> {
        let new_textures: Vec<Result<Rgba32FImage>> = paths
            .par_iter() // Parallel iterator
            .map(|&path| {
                // Load and decode each texture in parallel
                Ok(ImageReader::open(path)?.decode()?.into_rgba32f())
            })
            .collect(); // Collect results into a Vec

        for texture in new_textures {
            self.textures.push(texture?);
        }

        Ok(())
    }

    // to avoid repetitions, this is more efficient
    // returns (diff, emissive)
    // FIX: this code is bad
    pub fn get_colors(
        &self,
        u: f32,
        v: f32,
        geom: &Geometry,
        prim_id: u32,
    ) -> (LinearRgba, LinearRgba) {
        if let Texture::Solid(diff) = geom.material.texture {
            if let Texture::Solid(emissive) = geom.material.emissive {
                return (diff, emissive);
            }
        }

        // one of the textures is not a solid color
        // my goal was to only calculate this when needed
        // but the sampling should also have some unneeded repetitions
        let uv = geom.compute_uv(u, v, prim_id);

        let diff: LinearRgba = match geom.material.texture {
            Texture::Solid(diff) => diff,
            Texture::Image(id) => Self::sample(uv, &self.textures[id as usize]),
        };

        let emissive: LinearRgba = match geom.material.emissive {
            Texture::Solid(emissive) => emissive,
            Texture::Image(id) => Self::sample(uv, &self.textures[id as usize]),
        };

        (diff, emissive)
    }

    fn sample(uv: Vec2, texture: &Rgba32FImage) -> LinearRgba {
        let color = image::imageops::sample_bilinear(texture, uv.x, uv.y).expect("UV is in bounds");

        LinearRgba::rgb(color[0], color[1], color[2])
    }
}
