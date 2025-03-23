use crate::geometry::{GeomInfo, Geometry};
use crate::raytracer::{GeometryId, Ray, RayHitResult, RayTracer, RayTracerBuilder};
use embree4_rs::geometry::SphereGeometry;
use embree4_sys::{RTCRay, RTCRayHit};
use glam::Vec3;

pub struct EmbreeRayTracerBuilder<'a> {
    scene: embree4_rs::Scene<'a>,
    device: &'a embree4_rs::Device,
}

impl<'a> EmbreeRayTracerBuilder<'a> {
    pub fn new(device: &'a embree4_rs::Device) -> EmbreeRayTracerBuilder<'a> {
        let scene = embree4_rs::Scene::try_new(
            device,
            embree4_rs::SceneOptions {
                build_quality: embree4_sys::RTCBuildQuality::HIGH,
                flags: embree4_sys::RTCSceneFlags::ROBUST,
            },
        )
        .unwrap();

        EmbreeRayTracerBuilder { scene, device }
    }
}

impl RayTracerBuilder for EmbreeRayTracerBuilder<'_> {
    fn add_geometry(&mut self, geometry: &Geometry) -> anyhow::Result<GeometryId> {
        Ok(GeometryId(match &geometry.info {
            GeomInfo::Mesh(mesh) => {
                let embree_mesh = embree4_rs::geometry::TriangleMeshGeometry::try_new(
                    &self.device,
                    &mesh.verts,
                    &mesh.indices,
                )?;
                self.scene.attach_geometry(&embree_mesh)?
            }
            GeomInfo::Sphere(sphere) => {
                let embree_geom = SphereGeometry::try_new(
                    &self.device,
                    (sphere.center.x, sphere.center.y, sphere.center.z),
                    sphere.radius,
                )?;
                self.scene.attach_geometry(&embree_geom)?
            }
        }))
    }

    fn build(&self) -> anyhow::Result<impl RayTracer> {
        let committed_scene = self.scene.commit()?;
        Ok(EmbreeRayTracer { committed_scene })
    }
}

pub struct EmbreeRayTracer<'a> {
    committed_scene: embree4_rs::CommittedScene<'a>,
}

impl Into<RTCRay> for Ray {
    fn into(self) -> RTCRay {
        RTCRay {
            org_x: self.origin.x,
            org_y: self.origin.y,
            org_z: self.origin.z,
            dir_x: self.direction.x,
            dir_y: self.direction.y,
            dir_z: self.direction.z,
            tfar: self.max_distance,
            ..Default::default()
        }
    }
}

impl Into<RayHitResult> for RTCRayHit {
    fn into(self) -> RayHitResult {
        let origin = Vec3::new(self.ray.org_x, self.ray.org_y, self.ray.org_z);
        let dir = Vec3::new(self.ray.dir_x, self.ray.dir_y, self.ray.dir_z).normalize();
        let hit_point = origin + dir * self.ray.tfar;
        RayHitResult {
            hit_point,
            normal: Vec3::new(self.hit.Ng_x, self.hit.Ng_y, self.hit.Ng_z).normalize(),
            u: self.hit.u,
            v: self.hit.v,
            geometry_id: GeometryId(self.hit.geomID),
            triangle_id: self.hit.primID,
        }
    }
}

impl RayTracer for EmbreeRayTracer<'_> {
    fn intersect(&self, ray: Ray) -> Option<RayHitResult> {
        self.committed_scene
            .intersect_1(ray.into())
            .expect("Device error while intersecting ray")
            .map(Into::into)
    }
}
