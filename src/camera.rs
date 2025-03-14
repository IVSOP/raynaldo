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

    pub background: LinearRgba,
}

// pub struct RayInfo {
//     pub x: u32,
//     pub y: u32,
//     pub depth: u32,
// }

const DEPTH: u32 = 3;
const EPSILON: f32 = 1e-3;


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
            background: LinearRgba::new(0.1, 0.1, 0.8, 1.0),
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
    fn trace<'a>(&self, ray: RTCRay, scene: &CommittedScene<'a>, meshes: &MeshStorage, lights: &LightStorage, depth: u32) -> LinearRgba {

        if depth > 3 {
            return self.background
        }

        match scene.intersect_1(ray).unwrap() {
            Some(hit) => {
                let mut color = LinearRgba::BLACK;
                let mesh: &Mesh = meshes.get(hit.hit.geomID).unwrap();
                let material = &mesh.material;

                // if material.is_light {
                //     return material.emissive
                // }

                let origin = Vec3::new(hit.ray.org_x, hit.ray.org_y, hit.ray.org_z);
                let dir = Vec3::new(hit.ray.dir_x, hit.ray.dir_y, hit.ray.dir_z);
                // // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! A NORMAL NAO VEM NECESSARIAMENTE NORMALIZADA
                let normal = Vec3::new(hit.hit.Ng_x, hit.hit.Ng_y, hit.hit.Ng_z).normalize();
                // // println!("hit at {} with normal {} and color {}", origin + (dir * hit.ray.tfar), normal, mesh.material.color);
                let hit_pos = origin + dir * hit.ray.tfar;

                color += self.direct_lighting(hit_pos, normal, dir, &material, lights, scene);

                // TODO: I just ported over the depth checks. aren't they inneficient??????
                if depth < DEPTH {
                    let spec = material.specular;
                    if spec.red > 0.0 && spec.green > 0.0 && spec.blue > 0.0 {
                        color += self.specular_reflection(hit_pos, dir, normal, material, depth, scene, meshes, lights);
                    }
                }

                // material.color
                color
            },
            None => self.background
        }
    }

    pub fn render_pixel<'a>(&self, x: u32, y: u32, scene: &CommittedScene<'a>, meshes: &MeshStorage, lights: &LightStorage, spp: u32) -> LinearRgba {
        let mut base_color = LinearRgba::BLACK;
        let sppf = 1.0 / spp as f32;

        for _ in 0..spp {
            let ray = self.generate_ray(x, y, Some((fastrand::f32(), fastrand::f32())));

            base_color += self.trace(ray, scene, meshes, lights, 0) * sppf;
        }

        base_color
    }

    pub fn render<'a>(&self, image: &mut Rgb32FImage, scene: &CommittedScene<'a>, meshes: &MeshStorage, lights: &LightStorage, spp: u32) {
        // TODO: paralelize this. image access causes problems
        for y in 0..self.h {
            for x in 0..self.w {
                let color = self.render_pixel(x, y, scene, meshes, lights, spp);
                *image.get_pixel_mut(x, y) = Rgb::<f32>([color.red, color.green, color.blue]);
            }
        }
    }

    pub fn handle_ambient_light(&self, material: &Material, light: &Light) -> LinearRgba {
        if material.color.red > 0.0 && material.color.green > 0.0 && material.color.blue > 0.0 {
            LinearRgba::rgb(
                light.color.red * material.color.red,
                light.color.green * material.color.green,
                light.color.blue * material.color.blue,
            )
        } else {
            LinearRgba::BLACK
        }
    }

    pub fn handle_point_light<'a>(&self, material: &Material, light: &Light, hit_pos: Vec3, normal: Vec3, dir: Vec3, light_pos: Vec3, scene: &CommittedScene<'a>) -> LinearRgba {
        if material.diffuse.red > 0.0 && material.diffuse.green > 0.0 && material.diffuse.blue > 0.0 {
            // compiler please take care of this
            let distance_to_light = (light_pos - hit_pos).length();
            let dir_to_light = (light_pos - hit_pos).normalize();

            let light_cos = dir_to_light.dot(normal);
            if light_cos > 0.0 {

                // make a ray to the light source to check if there is a clear path from the hit position to the light
                // if there is, add light contribution

                let mut offset = EPSILON * normal;
                if dir_to_light.dot(normal) < 0.0 {
                    offset *= -1.0;
                }
                let shadow_ray_origin = hit_pos + offset;

                let shadow_ray = RTCRay {
                    org_x: shadow_ray_origin.x,
                    org_y: shadow_ray_origin.y,
                    org_z: shadow_ray_origin.z,
                    dir_x: dir_to_light.x,
                    dir_y: dir_to_light.y,
                    dir_z: dir_to_light.z,
                    tfar: distance_to_light - EPSILON,
                    ..default()
                };

                if let Some(_) = scene.intersect_1(shadow_ray).unwrap() {
                    let color = LinearRgba::rgb(
                        light.color.red * material.color.red,
                        light.color.green * material.color.green,
                        light.color.blue * material.color.blue,
                    ) * light_cos;

                    // println!("{:?}", color);
                    return color
                }
            }
        }
    
        LinearRgba::BLACK
    }

    // TODO: color * color e tao cursed que a bevy_color nem sequer implementa. mato-me?
    pub fn direct_lighting<'a>(&self, hit_pos: Vec3, normal: Vec3, dir: Vec3, material: &Material, lights: &LightStorage, scene: &CommittedScene<'a>) -> LinearRgba {
        let mut color = LinearRgba::BLACK;

        // loop over all light sources
        for light in lights.lights.iter() {
            color += match light.light_type {
                LightType::AMBIENT => {
                    self.handle_ambient_light(material, light)
                },
                LightType::POINT(light_pos) => {
                    self.handle_point_light(material, light, hit_pos, normal, dir, light_pos, scene)
                },
            }
        }

        color
    }

    // TODO: color * color e tao cursed que a bevy_color nem sequer implementa. mato-me?
    pub fn specular_reflection<'a>(&self, hit: Vec3, dir: Vec3, normal: Vec3, material: &Material, depth: u32, scene: &CommittedScene<'a>, meshes: &MeshStorage, lights: &LightStorage) -> LinearRgba {
        let rdir = dir.reflect(normal);

        let mut offset = EPSILON * normal;
        if rdir.dot(normal) < 0.0 {
            offset *= -1.0;
        }
        let rorign = hit + offset;

        // println!("ray originating at {} with dir {} reflected with dir {} and pos {}", origin, dir, rdir, rorign);

        let new_ray = RTCRay {
            org_x: rorign.x,
            org_y: rorign.y,
            org_z: rorign.z,
            dir_x: rdir.x,
            dir_y: rdir.y,
            dir_z: rdir.z,
            ..default()
        };

        let color = self.trace(new_ray, scene, meshes, lights, depth + 1);

        LinearRgba::rgb(
            material.specular.red * color.red,
            material.specular.green * color.green,
            material.specular.blue * color.blue,
        )
    }
}
