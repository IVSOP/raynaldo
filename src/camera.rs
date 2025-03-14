use glam::*;
use crate::ray::*;

pub struct Camera {
    pub pos: Vec3,
    pub up: Vec3,
    pub at_point: Vec3,

    pub pixel00_loc: Vec3,    // Location of pixel 0, 0
    pub pixel_delta_u: f32, // Offset to pixel to the right
    pub pixel_delta_v: f32, // Offset to pixel below

    pub tan_halfh: f32,
    pub w: u32,
    pub h: u32,
}

impl Camera {
    pub fn new(pos: Vec3, at_point: Vec3, up: Vec3, w_u32: u32, h_u32: u32, h_fov: f32) -> Self {

        let w = w_u32 as f32;
        let h = h_u32 as f32;

        let forward = (at_point - pos).normalize();
        let right = forward.cross(up).normalize();
        // recompute UP exactly as the cross product  right X forward
        let up = right.cross(forward).normalize();

        // Determine viewport dimensions.
        // precompute the tangents
        let tan_halfh = (h_fov / 2.0).tan();
        let vp_h = 2.0 * tan_halfh;
        let vp_w = vp_h * (w / h);

        let vp_u = vp_w / w;
        let vp_v = vp_h / h;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = vp_u / w;
        let pixel_delta_v = vp_v / h;

        // Calculate the location of the upper left pixel.
        let vp_upper_left = (pos + forward) - ((vp_u / 2.0) + (vp_h / 2.0));

        let pixel00_loc = vp_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            pos,
            up,
            at_point,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            tan_halfh,
            w: w_u32,
            h: h_u32,
        }
    }

    // should probably return a new ray, but won't for now to avoid differences while porting
    pub fn generateRay(&self, x: u32, y: f32, jitter: Option<(f32, f32)>, ray: &mut Ray) {
        let mut pc = Vec3::ZERO;

        if let Some((j1, j2)) = jitter {
            pc.x = (x as f32) + j1;
            pc.y = (x as f32) + j2;
        } else {
            pc.x = (x as f32) + 0.5;
            pc.y = (x as f32) + 0.5;
        }

        let pixel_sample = self.pixel00_loc + (pc.x * self.pixel_delta_u) + (pc.y * self.pixel_delta_v);

        ray.origin = self.pos;
        ray.dir = (pixel_sample - self.pos).normalize();
        ray.invert_dir();
        ray.pix_x = x;
        ray.face_id = -1;
        ray.propagating_eta = 1.0;
    }
}
