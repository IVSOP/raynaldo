use image::ImageReader;
use image::Rgba32FImage;
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

    // retrieves the base color from a texture
    // u v are actually the uv passed in by embree
    // usually when this is called I already have the material in scope, just pass it in?
    pub fn get_color(&self, u: f32, v: f32, geom: &Geometry, prim_id: u32) -> LinearRgba {
        match geom.material.texture {
            Texture::Solid(color) => color,
            Texture::Image(id) => {
                let texture = self.textures.get(id as usize).unwrap();
                geom.get_color(u, v, prim_id, texture)
            }
        }
    }

    pub fn get_emissive(&self, u: f32, v: f32, geom: &Geometry, prim_id: u32) -> LinearRgba {
        match geom.material.emissive {
            Texture::Solid(color) => color,
            Texture::Image(id) => {
                let texture = self.textures.get(id as usize).unwrap();
                geom.get_color(u, v, prim_id, texture)
            }
        }
    }
}
