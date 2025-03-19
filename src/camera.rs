use crate::common::*;
use crate::consts::*;
use crate::geometry::*;
use bevy_color::LinearRgba;
use bevy_math::*;
use embree4_rs::*;
use embree4_sys::RTCRay;
use image::Rgb;
use image::Rgb32FImage;
use rayon::iter::*;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Camera {
    pub pos: Vec3,
    // pub up: Vec3,
    // pub at_point: Vec3,
    pub pixel00_loc: Vec3,   // Location of pixel 0, 0
    pub pixel_delta_u: Vec3, // Offset to pixel to the right
    pub pixel_delta_v: Vec3, // Offset to pixel below

    // pub tan_halfh: f32,
    pub w: u32,
    pub h: u32,

    pub background: LinearRgba,
}

// the rays need to know the refraction index of where they start
// I use the 32 bits of the id to encode this information
// WARNING: if more information is needed in the future, this will no longer have enough space
// will probably need to use the id to index into some data structure
pub struct RayInfo {
    pub refraction: f32,
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

        let pixel_sample =
            self.pixel00_loc + (pc.x * self.pixel_delta_u) + (pc.y * self.pixel_delta_v);
        let dir = (pixel_sample - self.pos).normalize();

        let info = RayInfo {
            refraction: AIR_REFRACT,
        };

        // Ray::new(self.pos, dir, LinearRgba::BLACK, x, y, 1.0)
        RTCRay {
            org_x: self.pos.x,
            org_y: self.pos.y,
            org_z: self.pos.z,
            dir_x: dir.x,
            dir_y: dir.y,
            dir_z: dir.z,
            id: info.refraction.to_bits(),
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
    fn trace(
        &self,
        ray: RTCRay,
        scene: &CommittedScene<'_>,
        store: &Storage,
        depth: u32,
    ) -> LinearRgba {
        if depth > Consts::DEPTH {
            return self.background;
        }

        match scene.intersect_1(ray).unwrap() {
            Some(hit) => {
                let mut color = LinearRgba::BLACK;
                let geometry: &Geometry = store.get_geometry(hit.hit.geomID).unwrap();
                let material = &geometry.material;

                let origin = Vec3::new(hit.ray.org_x, hit.ray.org_y, hit.ray.org_z);
                let dir = Vec3::new(hit.ray.dir_x, hit.ray.dir_y, hit.ray.dir_z);
                // // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! A NORMAL NAO VEM NECESSARIAMENTE NORMALIZADA
                let normal = Vec3::new(hit.hit.Ng_x, hit.hit.Ng_y, hit.hit.Ng_z).normalize();
                // // println!("hit at {} with normal {} and color {}", origin + (dir * hit.ray.tfar), normal, mesh.material.color);
                let hit_pos = origin + dir * hit.ray.tfar;
                let refraction = f32::from_bits(hit.ray.id);
                let u = hit.hit.u;
                let v = hit.hit.v;
                let prim_id = hit.hit.primID;

                let (diff, emissive) = store.get_colors(u, v, geometry, prim_id);

                color += self.direct_lighting(hit_pos, normal, &material, diff, scene, store);

                color += self.reflect_refract(
                    hit_pos, dir, normal, diff, refraction, material, depth, scene, store,
                );

                color += emissive;

                // let spec = material.specular;
                // if spec.red > 0.0 || spec.green > 0.0 || spec.blue > 0.0 {
                //     color += self.specular_reflection(
                //         hit_pos, dir, normal, refraction, material, depth, scene, store, lights,
                //     );
                // }

                // let transmission = material.transmission;
                // if transmission.red > 0.0 || transmission.green > 0.0 || transmission.blue > 0.0
                // {
                //     color += self.specular_transmission(
                //         hit_pos, dir, normal, refraction, material, depth, scene, store, lights,
                //     );
                // }

                // material.color
                color
            }
            None => self.background,
        }
    }

    pub fn render_pixel(
        &self,
        x: u32,
        y: u32,
        scene: &CommittedScene<'_>,
        store: &Storage,
    ) -> LinearRgba {
        let mut base_color = LinearRgba::BLACK;

        for _ in 0..Consts::RAYS_PER_PIXEL {
            let ray = self.generate_ray(x, y, Some((fastrand::f32(), fastrand::f32())));

            base_color += self.trace(ray, scene, store, 0) * (1.0 / Consts::RAYS_PER_PIXEL as f32);
        }

        base_color
    }

    pub fn render(&self, image: &mut Rgb32FImage, scene: &CommittedScene<'_>, store: &Storage) {
        // TODO: add more unsafe to be faster
        let image = Mutex::new(image);
        (0..self.h).into_par_iter().for_each(|y| {
            // wtf this is terrible
            let mut image_slice: Vec<Rgb<f32>> = vec![Rgb::<f32>([0.0, 0.0, 0.0]); self.w as usize];
            for x in 0..self.w {
                let color = self.render_pixel(x, y, scene, store);
                *image_slice.get_mut(x as usize).unwrap() =
                    Rgb::<f32>([color.red, color.green, color.blue]);
            }

            let mut img = image.lock().unwrap();
            for x in 0..self.w {
                *img.get_pixel_mut(x, y) = *image_slice.get(x as usize).unwrap();
            }
        });
    }

    // pass in material to save some work???
    pub fn handle_ambient_light(&self, material: &Material, light: &Light) -> LinearRgba {
        LinearRgba::rgb(
            light.color.red * material.color.red,
            light.color.green * material.color.green,
            light.color.blue * material.color.blue,
        )
    }

    pub fn handle_point_light(
        &self,
        diff: LinearRgba,
        light: &Light,
        hit_pos: Vec3,
        normal: Vec3,
        light_pos: Vec3,
        scene: &CommittedScene<'_>,
    ) -> LinearRgba {
        // if material.diffuse.red > 0.0 || material.diffuse.green > 0.0 || material.diffuse.blue > 0.0
        // {
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

            // we have a direct path to the light, can add direct illumination
            if scene.intersect_1(shadow_ray).unwrap().is_none() {
                let color = LinearRgba::rgb(
                    light.color.red * diff.red,
                    light.color.green * diff.green,
                    light.color.blue * diff.blue,
                ) * light_cos;

                // println!("{:?}", color);
                return color;
            }
        }
        // }

        LinearRgba::BLACK
    }

    // randomly select N points on the light and make them act as individual point lights
    pub fn handle_square_light(
        &self,
        diff: LinearRgba,
        light: &Light,
        square: &LightQuad,
        hit_pos: Vec3,
        normal: Vec3,
        scene: &CommittedScene<'_>,
    ) -> LinearRgba {
        // if material.diffuse.red > 0.0 || material.diffuse.green > 0.0 || material.diffuse.blue > 0.0
        // {
        let mut color = LinearRgba::BLACK;
        for _ in 0..Consts::NUM_AREA_LIGHT_TESTS {
            let u = fastrand::f32();
            let v = fastrand::f32();

            let light_pos = square.bottom_left + (u * square.u_vec) + (v * square.v_vec);

            // all the logic here is copied from point lights except NUM_AREA_LIGHT_TESTS

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

                // we have a direct path to the light, can add direct illumination
                if scene.intersect_1(shadow_ray).unwrap().is_none() {
                    color += LinearRgba::rgb(
                        light.color.red * diff.red,
                        light.color.green * diff.green,
                        light.color.blue * diff.blue,
                    ) * light_cos
                        * (1.0 / Consts::NUM_AREA_LIGHT_TESTS as f32);

                    // println!("{:?}", color);
                }
            }
        }

        color
    }

    // TODO: color * color e tao cursed que a bevy_color nem sequer implementa. mato-me?
    pub fn direct_lighting(
        &self,
        hit_pos: Vec3,
        normal: Vec3,
        material: &Material,
        diff: LinearRgba,
        scene: &CommittedScene<'_>,
        store: &Storage,
    ) -> LinearRgba {
        let mut color = LinearRgba::BLACK;

        let lights = &store.lights;

        if Consts::COMPARE_ALL_LIGHTS {
            // loop over all light sources
            for light in lights.iter() {
                color += match light.light_type {
                    LightType::Ambient => self.handle_ambient_light(material, light),
                    LightType::Point(light_pos) => {
                        self.handle_point_light(diff, light, hit_pos, normal, light_pos, scene)
                    }
                    LightType::AreaQuad(ref square) => {
                        self.handle_square_light(diff, light, square, hit_pos, normal, scene)
                    }
                };
            }
        } else {
            let light_i = fastrand::usize(..lights.len());
            let light = lights.get(light_i).unwrap();
            color += match light.light_type {
                LightType::Ambient => self.handle_ambient_light(material, light),
                LightType::Point(light_pos) => {
                    self.handle_point_light(diff, light, hit_pos, normal, light_pos, scene)
                }
                LightType::AreaQuad(ref square) => {
                    self.handle_square_light(diff, light, square, hit_pos, normal, scene)
                }
            };

            color *= lights.len() as f32;
        }

        color
    }

    // // TODO: color * color e tao cursed que a bevy_color nem sequer implementa. mato-me?
    // pub fn specular_reflection(
    //     &self,
    //     hit: Vec3,
    //     dir: Vec3,
    //     normal: Vec3,
    //     ray_refraction: f32,
    //     material: &Material,
    //     depth: u32,
    //     scene: &CommittedScene<'_>,
    //     store: &Storage,
    // ) -> LinearRgba {
    //     let rdir = dir.reflect(normal);

    //     let mut offset = EPSILON * normal;
    //     if rdir.dot(normal) < 0.0 {
    //         offset *= -1.0;
    //     }
    //     let rorign = hit + offset;

    //     // println!("ray originating at {} with dir {} reflected with dir {} and pos {}", origin, dir, rdir, rorign);

    //     let info = RayInfo {
    //         refraction: ray_refraction, // the medium did not change
    //     };

    //     let new_ray = RTCRay {
    //         org_x: rorign.x,
    //         org_y: rorign.y,
    //         org_z: rorign.z,
    //         dir_x: rdir.x,
    //         dir_y: rdir.y,
    //         dir_z: rdir.z,
    //         id: info.refraction.to_bits(),
    //         ..default()
    //     };

    //     let color = self.trace(new_ray, scene, store, lights, depth + 1);

    //     LinearRgba::rgb(
    //         material.specular.red * color.red,
    //         material.specular.green * color.green,
    //         material.specular.blue * color.blue,
    //     )
    // }

    // // TODO: color * color e tao cursed que a bevy_color nem sequer implementa. mato-me?
    // // TODO cleanup
    // pub fn specular_transmission(
    //     &self,
    //     hit: Vec3,
    //     dir: Vec3,
    //     normal: Vec3,
    //     ray_refraction: f32,
    //     material: &Material,
    //     depth: u32,
    //     scene: &CommittedScene<'_>,
    //     store: &Storage,
    // ) -> LinearRgba {

    //     let material_refraction: f32;
    //     if ray_refraction == 1.0 {
    //         material_refraction = material.refraction;
    //     } else {
    //         material_refraction = 1.0;
    //     }

    //     let ior = ray_refraction / material_refraction;

    //     let cos_theta = normal.dot(dir).min(1.0);
    //     let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt(); // f64??

    //     // is there total internal reflection ?
    //     let cannot_refract: bool = ior * sin_theta > 1.0;

    //     let refdir = if cannot_refract {
    //         dir.reflect(normal).normalize()
    //     } else {
    //         dir.refract(normal, ior).normalize()
    //     };

    //     let info = if cannot_refract {
    //         // reflected, so it stays in the same medium
    //         RayInfo {
    //             refraction: ray_refraction,
    //         }
    //     } else {
    //         RayInfo {
    //             refraction: material_refraction,
    //         }
    //     };

    //     let inv_normal = -normal;
    //     let mut offset = EPSILON * inv_normal;
    //     if refdir.dot(inv_normal) < 0.0 {
    //         offset *= -1.0;
    //     }
    //     let origin = hit + offset;

    //     let refraction_ray = RTCRay {
    //         org_x: origin.x,
    //         org_y: origin.y,
    //         org_z: origin.z,
    //         dir_x: refdir.x,
    //         dir_y: refdir.y,
    //         dir_z: refdir.z,
    //         id: info.refraction.to_bits(),
    //         ..default()
    //     };

    //     let color = self.trace(refraction_ray, scene, store, lights, depth + 1);

    //     LinearRgba::rgb(
    //         material.transmission.red * color.red,
    //         material.transmission.green * color.green,
    //         material.transmission.blue * color.blue,
    //     )
    // }

    /// use schlick approximation to compute fraction of light specularly reflected at a surface (fresnel)
    pub fn compute_reflection_coeff(
        &self,
        incident_dir: Vec3,
        normal: Vec3,
        n1: f32, // refraction being left
        n2: f32, // refraction being entered
        material: &Material,
    ) -> f32 {
        let mut r0 = (n1 - n2) / (n1 + n2);
        r0 *= r0;
        let mut cos_x = -normal.dot(incident_dir);
        if n1 > n2 {
            let n = n1 / n2;
            let sin_t2 = n * n * (1.0 - cos_x * cos_x);
            // Total internal reflection
            if sin_t2 > 1.0 {
                return 1.0;
            }
            cos_x = (1.0 - sin_t2).sqrt();
        }
        let x = 1.0 - cos_x;
        let ret = r0 + (1.0 - r0) * x * x * x * x * x;

        material.reflectivity + (1.0 - material.reflectivity) * ret
    }

    pub fn reflect_refract(
        &self,
        hit: Vec3,
        incident_dir: Vec3,
        normal: Vec3,
        diff: LinearRgba,
        ray_refraction: f32,
        material: &Material,
        depth: u32,
        scene: &CommittedScene<'_>,
        store: &Storage,
    ) -> LinearRgba {
        // n1 is refraction being left
        // n2 is refraction being entered

        let n1: f32;
        let n2: f32;

        // WARNING nao tenho como saber se o raio esta a sair ou entrar no material,
        // nem qual o indice de refracao que devo usar ao sair, entao tive de fazer esta manhosice
        if ray_refraction == AIR_REFRACT {
            // ray is coming from outside, entering
            n1 = AIR_REFRACT;
            n2 = material.refraction;
        } else {
            // ray is leaving
            n1 = material.refraction;
            n2 = AIR_REFRACT;
        }

        let reflect = self.compute_reflection_coeff(incident_dir, normal, n1, n2, material);
        let refract = 1.0 - reflect;

        // REFLECTION, for materials with reflectivity
        let mut color = LinearRgba::BLACK;
        if reflect > 0.0 && material.reflectivity > 0.0 {
            let refdir = incident_dir.reflect(normal);

            let mut offset = EPSILON * normal;
            if refdir.dot(normal) < 0.0 {
                offset *= -1.0;
            }
            let origin = hit + offset;

            let refraction = n1; // medium did not change

            let reflection_ray = RTCRay {
                org_x: origin.x,
                org_y: origin.y,
                org_z: origin.z,
                dir_x: refdir.x,
                dir_y: refdir.y,
                dir_z: refdir.z,
                id: refraction.to_bits(),
                ..default()
            };

            color += self.trace(reflection_ray, scene, store, depth + 1);

            color = LinearRgba::rgb(
                material.specular.red * color.red,
                material.specular.green * color.green,
                material.specular.blue * color.blue,
            ) * reflect; // no need to multiply by reflectivity of the material, fresnel already takes it into account
        }

        // REFRACTION, for materials with transparency
        if refract > 0.0 && material.transparency > 0.0 {
            let eta = n1 / n2; // Ratio of IORs
            let facing_normal = if incident_dir.dot(normal) < 0.0 {
                normal
            } else {
                -normal
            };
            let refract_dir = incident_dir.refract(facing_normal, eta);

            if refract_dir != Vec3::ZERO {
                // Refraction succeeded (no TIR)
                // Offset the origin to avoid self-intersection
                let mut offset = EPSILON * facing_normal;
                if refract_dir.dot(facing_normal) < 0.0 {
                    offset *= -1.0;
                }
                let origin = hit + offset;

                let refraction = n2; // New medium’s IOR

                let refraction_ray = RTCRay {
                    org_x: origin.x,
                    org_y: origin.y,
                    org_z: origin.z,
                    dir_x: refract_dir.x,
                    dir_y: refract_dir.y,
                    dir_z: refract_dir.z,
                    id: refraction.to_bits(),
                    ..default()
                };

                color += self.trace(refraction_ray, scene, store, depth + 1);

                color = LinearRgba::rgb(
                    material.transmission.red * color.red,
                    material.transmission.green * color.green,
                    material.transmission.blue * color.blue,
                ) * material.transparency
                    * refract;
                // here I use transparency to imply some of the light is lost/absorbed when refracting
                // this is technically not needed and not correct, but beer's law is not yet implemented so this will have to do for now
            }
        }

        // SCATTERING
        // WARN: limited to the first ever hit
        // uses what's left after reflection (like refraction), but assumes the material does not refract
        if depth == 0 {
            let diffuse_strength = 1.0; // TEMPORARY
            let diffuse_weight = if material.transparency > 0.0 {
                0.0
            } else {
                (1.0 - reflect) * diffuse_strength // Diffuse takes what's left after reflection
            };

            if diffuse_weight > 0.0 {
                for _ in 0..Consts::NUM_SCATTER {
                    let scatter_dir = sample_cos_hemisphere(normal);
                    let offset = EPSILON * normal;
                    let origin = hit + offset;
                    let scatter_ray = RTCRay {
                        org_x: origin.x,
                        org_y: origin.y,
                        org_z: origin.z,
                        dir_x: scatter_dir.x,
                        dir_y: scatter_dir.y,
                        dir_z: scatter_dir.z,
                        id: n1.to_bits(), // Stays in same medium, it's kind of a second reflection
                        ..default()
                    };
                    let scattered_color = self.trace(scatter_ray, scene, store, depth + 1);
                    let cos_theta = scatter_dir.dot(normal).max(0.0);
                    // TODO what color from the current material should I be using???
                    color += LinearRgba::rgb(
                        scattered_color.red * diff.red,
                        scattered_color.green * diff.green,
                        scattered_color.blue * diff.blue,
                    ) * diffuse_weight
                        * cos_theta
                        * (1.0 / Consts::NUM_SCATTER as f32);
                }
            }
        }

        color
    }
}

fn sample_cos_hemisphere(normal: Vec3) -> Vec3 {
    // Two random numbers in [0, 1)
    let e1 = fastrand::f32();
    let e2 = fastrand::f32();

    // Cosine-weighted sampling in local space (normal = (0, 0, 1))
    let r = e1.sqrt(); // Radius on the unit disk
    let phi = 2.0 * std::f32::consts::PI * e2; // Azimuthal angle
    let x = r * phi.cos();
    let y = r * phi.sin();
    let z = (1.0 - e1).sqrt(); // Ensures unit length and cosine weighting

    // Local direction
    let local_dir = Vec3::new(x, y, z);

    // Transform to world space using an orthonormal basis
    let (u, v, w) = orthonormal_basis(normal);
    u * local_dir.x + v * local_dir.y + w * local_dir.z
}

fn orthonormal_basis(normal: Vec3) -> (Vec3, Vec3, Vec3) {
    let w = normal; // Normal is already normalized
    let a = if w.x.abs() > 0.9 { Vec3::Y } else { Vec3::X }; // Avoid parallel vectors
    let v = w.cross(a).normalize();
    let u = w.cross(v).normalize();
    (u, v, w)
}
