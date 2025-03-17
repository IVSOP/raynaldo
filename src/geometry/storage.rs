use image::Rgb32FImage;
use std::collections::HashMap;

use super::*;

pub struct Storage {
    pub lights: Vec<Light>,
    pub geom: HashMap<u32, Geometry>,
    pub textures: Vec<Rgb32FImage>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            lights: Vec::new(),
            geom: HashMap::new(),
            textures: Vec::new(),
        }
    }
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
}
