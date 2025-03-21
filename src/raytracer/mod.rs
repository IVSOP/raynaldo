use bevy_math::Vec3;

pub mod embree;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub max_distance: f32,
}

pub struct RayHitResult {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub u: f32,
    pub v: f32,
    /// The identifier of the hit geometry.
    pub geometry_id: GeometryId,
    /// The index of the triangle hit in the geometry.
    pub triangle_id: u32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self::new_with_max_distance(origin, dir, f32::INFINITY)
    }

    pub fn new_with_max_distance(origin: Vec3, direction: Vec3, max_distance: f32) -> Self {
        Self {
            origin,
            direction,
            max_distance,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct GeometryId(u32);

pub trait RayTracerBuilder {
    fn add_geometry(&mut self, geometry: &crate::geometry::Geometry) -> anyhow::Result<GeometryId>;
    fn build(&self) -> anyhow::Result<impl RayTracer>;
}

pub trait RayTracer {
    fn intersect(&self, ray: Ray) -> Option<RayHitResult>;
}
