use crate::configs::CamConfig;
use crate::raytracer::Ray;
use glam::Vec3;

#[derive(Debug)]
pub struct Camera {
    pub config: CamConfig,
    // pub up: Vec3,
    // pub at_point: Vec3,
    /// Location of pixel 0, 0
    pub pixel00_loc: Vec3,
    /// Offset to pixel to the right
    pub pixel_delta_u: Vec3,
    /// Offset to pixel below
    pub pixel_delta_v: Vec3,
    // pub tan_halfh: f32,
}

impl Camera {
    pub fn new(config: CamConfig) -> Self {
        let w = config.w as f32;
        let h = config.h as f32;

        const UP: Vec3 = Vec3::Y;

        let forward = (config.lookat - config.pos).normalize();
        let right = forward.cross(UP).normalize();
        // recompute UP exactly as the cross product  right X forward
        let up = right.cross(forward).normalize();

        // Determine viewport dimensions.
        // precompute the tangents
        let tan_halfh = (config.fov / 2.0).tan();
        let vp_h = 2.0 * tan_halfh;
        let vp_w = vp_h * (w / h);

        let vp_u = vp_w * right;
        let vp_v = -vp_h * up;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = vp_u / w;
        let pixel_delta_v = vp_v / h;

        // Calculate the location of the upper left pixel.
        let vp_upper_left = (config.pos + forward) - ((vp_u / 2.0) + (vp_v / 2.0));

        let pixel00_loc = vp_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // println!("{:#?}", Self {
        //     pos,
        //     up,
        //     at_point,
        //     pixel00_loc,
        //     pixel_delta_u,
        //     pixel_delta_v,
        //     tan_halfh,
        //     w: w_u32,
        //     h: h_u32,
        // });
        // panic!();

        Self {
            config,
            // up,
            // at_point,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            // tan_halfh,
            // TODO: REMOVE
            // background: LinearRgba::new(0.1, 0.1, 0.8, 1.0),
        }
    }

    pub fn generate_ray(&self, x: u32, y: u32, (j1, j2): (f32, f32)) -> Ray {
        let pc = Vec3::new((x as f32) + j1, (y as f32) + j2, 0.0);

        let pixel_sample =
            self.pixel00_loc + (pc.x * self.pixel_delta_u) + (pc.y * self.pixel_delta_v);
        let dir = (pixel_sample - self.config.pos).normalize();

        Ray::new(self.config.pos, dir)
    }
}
