use bevy_color::LinearRgba;
use glam::*;

const EPSILON: f32 = 1e-3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub face_id: i32, // ID of the face where the origin lays in
    pub inv_dir: Vec3, // ray direction reciprocal for intersections
    pub throughput: LinearRgba,
    pub pix_x: u32,
    pub pix_y: u32,
    pub propagating_eta: f32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, throughput: LinearRgba, pix_x: u32, pix_y: u32, propagating_eta: f32) -> Self {
        let inv_dir = 1.0 / dir;
        Self {
            origin,
            dir,
            throughput,
            inv_dir,
            pix_x,
            pix_y,
            propagating_eta,
            face_id: -1,
        }
    }

    pub fn invert_dir(&mut self) {
        self.inv_dir = 1.0 / self.dir;
    }

    pub fn adjust_origin(&mut self, normal: Vec3) {
        let mut offset = EPSILON * normal;
        if self.dir.dot(normal) < 0.0 {
            offset = -1.0 * offset;
        }

        self.origin += offset;
    }
}
