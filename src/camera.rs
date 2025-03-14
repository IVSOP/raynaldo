use bevy_color::LinearRgba;
use glam::*;
use image::Rgb32FImage;
use embree4_rs::*;
use embree4_sys::RTCRay;
use crate::mesh::*;
use crate::common::*;
use image::Rgb;

#[derive(Debug)]
pub struct Camera {
    pub pos: Vec3,
    // pub up: Vec3,
    // pub at_point: Vec3,

    pub pixel00_loc: Vec3,    // Location of pixel 0, 0
    pub pixel_delta_u: Vec3, // Offset to pixel to the right
    pub pixel_delta_v: Vec3, // Offset to pixel below

    // pub tan_halfh: f32,
    pub w: u32,
    pub h: u32,
}

// pub struct RayInfo {
//     pub x: u32,
//     pub y: u32,
//     pub depth: u32,
// }

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

        let vp_u = vp_w * right;
        let vp_v = -vp_h * up;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = vp_u / w;
        let pixel_delta_v = vp_v / h;

        // Calculate the location of the upper left pixel.
        let vp_upper_left = (pos + forward) - ((vp_u / 2.0) + (vp_v / 2.0));

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
            pos,
            // up,
            // at_point,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            // tan_halfh,
            w: w_u32,
            h: h_u32,
        }
    }

    pub fn generate_ray(&self, x: u32, y: u32, jitter: Option<(f32, f32)>) -> RTCRay {
        let mut pc = Vec3::ZERO;

        if let Some((j1, j2)) = jitter {
            pc.x = (x as f32) + j1;
            pc.y = (y as f32) + j2;
        } else {
            // position at center of pixel
            pc.x = (x as f32) + 0.5;
            pc.y = (y as f32) + 0.5;
        }

        let pixel_sample = self.pixel00_loc + (pc.x * self.pixel_delta_u) + (pc.y * self.pixel_delta_v);
        let dir = (pixel_sample - self.pos).normalize();

        // Ray::new(self.pos, dir, LinearRgba::BLACK, x, y, 1.0)
        RTCRay {
            org_x: self.pos.x,
            org_y: self.pos.y,
            org_z: self.pos.z,
            dir_x: dir.x,
            dir_y: dir.y,
            dir_z: dir.z,
            // id,
            // time: 10000000000.0,
            ..default()
        }
    }

    // pub fn compute_id(&self, x: u32, y: u32) -> u32 {
    //     x + (y * self.w)
    // }

    // pub fn compute_x_y(&self, index: u32) -> (u32, u32) {
    //     (index % self.w, index / self.w)
    // }

    // returns (total rays casted, hits)
    fn trace<'a>(&self, ray: RTCRay, scene: &CommittedScene<'a>, storage: &MeshStorage, sppf: f32, depth: u32) -> Option<LinearRgba> {

        match scene.intersect_1(ray).unwrap() {
            Some(hit) => {
                let origin = Vec3::new(hit.ray.org_x, hit.ray.org_y, hit.ray.org_z);
                let dir = Vec3::new(hit.ray.dir_x, hit.ray.dir_y, hit.ray.dir_z);
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! A NORMAL NAO VEM NECESSARIAMENTE NORMALIZADA
                let normal = Vec3::new(hit.hit.Ng_x, hit.hit.Ng_y, hit.hit.Ng_z);
                let mesh: &Mesh = storage.get(hit.hit.geomID).unwrap();
                // println!("hit at {} with normal {} and color {}", origin + (dir * hit.ray.tfar), normal, mesh.material.color);
                
                Some(mesh.material.color * sppf)
            },
            None => None,
        }
    }

    pub fn render_pixel<'a>(&self, x: u32, y: u32, scene: &CommittedScene<'a>, storage: &MeshStorage, spp: u32) -> LinearRgba {
        let mut base_color =  LinearRgba::BLACK;
        for _ in 0..spp {
            let ray = self.generate_ray(x, y, None);

            if let Some(color) = self.trace(ray, scene, storage, 1.0 / spp as f32, 0) {
                base_color += color;
            }
        }

        base_color
    }

    pub fn render<'a>(&self, image: &mut Rgb32FImage, scene: &CommittedScene<'a>, storage: &MeshStorage, spp: u32) {
        // TODO: paralelize this. image access causes problems
        for y in 0..self.h {
            for x in 0..self.w {
                let color = self.render_pixel(x, y, scene, storage, spp);
                *image.get_pixel_mut(x, y) = Rgb::<f32>([color.red, color.green, color.blue]);
            }
        }
    }
}
