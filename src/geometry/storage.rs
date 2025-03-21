use crate::geometry::{Geometry, Light, Texture};
use crate::raytracer::{GeometryId, RayTracer, RayTracerBuilder};
use anyhow::{Context, Result};
use bevy_color::LinearRgba;
use bevy_math::Vec2;
use fxhash::FxHashMap;
use image::ImageReader;
use image::Rgba32FImage;
use rayon::prelude::*;

pub struct Scene {
    pub lights: Vec<Light>,
    pub geometry: Vec<Geometry>,
    pub textures: Vec<Rgba32FImage>,
}

impl Scene {
    // use this instead of default so that the default texture is loaded
    pub fn new() -> Result<Self> {
        let mut res = Self {
            lights: Vec::new(),
            geometry: Vec::new(),
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

    pub fn add_geometry(&mut self, geom: Geometry) {
        self.geometry.push(geom);
    }

    // I assume opening the file is very fast but decoding() and into_rgba32f() are slow, so I just made the whole thing parallel
    // cursed, does not error out instantly
    pub fn add_textures_batch_from_files(&mut self, paths: &[&str]) -> Result<()> {
        let new_textures: Result<Vec<Rgba32FImage>> = paths
            .par_iter() // Parallel iterator
            .map(|&path| {
                // Load and decode each texture in parallel
                Ok(ImageReader::open(path)?.decode()?.into_rgba32f())
            })
            .collect(); // Collect results into a Vec

        let new_textures = new_textures.context("Error loading textures")?;

        for texture in new_textures {
            self.textures.push(texture);
        }

        Ok(())
    }

    pub fn build_scene(
        self,
        raytracer_builder: &mut impl RayTracerBuilder,
    ) -> Result<BuiltScene<impl RayTracer>> {
        let mut geometry_map = FxHashMap::default();

        for geometry in self.geometry {
            let id = raytracer_builder.add_geometry(&geometry)?;
            geometry_map.insert(id, geometry);
        }

        let raytracer = raytracer_builder.build()?;

        Ok(BuiltScene {
            lights: self.lights,
            geometry: geometry_map,
            textures: self.textures,
            raytracer,
        })
    }
}

pub struct BuiltScene<T: RayTracer> {
    pub lights: Vec<Light>,
    pub geometry: FxHashMap<GeometryId, Geometry>,
    pub textures: Vec<Rgba32FImage>,
    pub raytracer: T,
}

impl<T: RayTracer> BuiltScene<T> {
    pub fn get_geometry(&self, id: GeometryId) -> Option<&Geometry> {
        self.geometry.get(&id)
    }

    // to avoid repetitions, this is more efficient
    // returns (diff, emissive)
    // FIX: this code is bad
    pub fn sample_color(
        &self,
        geom: &Geometry,
        prim_id: u32,
        u: f32,
        v: f32,
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
            Texture::Image(id) => Self::sample_texture(uv, &self.textures[id as usize]),
        };

        let emissive: LinearRgba = match geom.material.emissive {
            Texture::Solid(emissive) => emissive,
            Texture::Image(id) => Self::sample_texture(uv, &self.textures[id as usize]),
        };

        (diff, emissive)
    }

    fn sample_texture(uv: Vec2, texture: &Rgba32FImage) -> LinearRgba {
        let color = image::imageops::sample_bilinear(texture, uv.x, uv.y).expect("UV is in bounds");

        LinearRgba::rgb(color[0], color[1], color[2])
    }
}
