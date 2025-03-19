use image::ImageReader;
use image::Rgba32FImage;
use rayon::prelude::*;
use std::collections::HashMap;

use super::*;

pub struct Storage {
    pub lights: Vec<Light>,
    pub geom: HashMap<u32, Geometry>,
    pub textures: Vec<Rgba32FImage>,
}

impl Storage {
    pub fn attach_geometry(
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

    pub fn get_geometry(&self, id: u32) -> Option<&Geometry> {
        return self.geom.get(&id);
    }

    // use this instead of default so that the default texture is loaded
    pub fn new() -> Self {
        let mut res = Self {
            lights: Vec::new(),
            geom: HashMap::new(),
            textures: Vec::new(),
        };

        let default_texture: Rgba32FImage = ImageReader::open("assets/textures/uv.png")
            .expect("Default texture (assets/textures/uv.png) does not exist")
            .decode()
            .expect("Error decoding texture assets/textures/uv.png")
            .into_rgba32f();

        res.textures.push(default_texture);

        res
    }

    // TODO retornar mensagens com o path????
    pub fn load_texture(&mut self, path: &str) -> Result<()> {
        let texture: Rgba32FImage = ImageReader::open(path)?.decode()?.into_rgba32f();
        // .expect(format!("texture {} does not exist", path).as_str())
        // .decode()
        // .expect(format!("Error decoding texture assets/textures/uv.png").as_str())
        // .into_rgba32f();

        self.textures.push(texture);

        Ok(())
    }

    // I assume opening the file is very fast but decoding() and into_rgba32f() are slow, so I just made the whole thing parallel
    // cursed, does not error out instantly
    pub fn load_textures_batch(&mut self, paths: Vec<&str>) -> Result<()> {
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
            Texture::Image(id) => sample(uv, &self.textures[id as usize]),
        };

        let emissive: LinearRgba = match geom.material.emissive {
            Texture::Solid(emissive) => emissive,
            Texture::Image(id) => sample(uv, &self.textures[id as usize]),
        };

        (diff, emissive)
    }
}
