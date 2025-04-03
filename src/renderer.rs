use crate::camera::Camera;
use crate::color::Rgba;
use crate::common::compute_reflection_coeff;
use crate::configs::RenderConfig;
use crate::geometry::{BuiltScene, Geometry, Light, LightQuad, LightType, Material};
use crate::raytracer::{Ray, RayTracer};
use glam::Vec3;
use image::{Rgb, Rgb32FImage};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

pub const EPSILON: f32 = 1e-3;
pub const AIR_REFRACT: f32 = 1.00029;

pub struct Renderer<T: RayTracer> {
    scene: BuiltScene<T>,
    config: RenderConfig,
}

impl<T: RayTracer + Sync> Renderer<T> {
    pub fn render_par(&self, camera: &Camera) -> Rgb32FImage {
        Rgb32FImage::from_par_fn(camera.config.w, camera.config.h, |x, y| {
            self.render_pixel(x, y, camera)
        })
    }

    pub fn render_par_with_progress(
        &self,
        camera: &Camera,
        progress: Arc<AtomicUsize>,
    ) -> Rgb32FImage {
        Rgb32FImage::from_par_fn(camera.config.w, camera.config.h, |x, y| {
            let result = self.render_pixel(x, y, camera);
            progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            result
        })
    }
}

impl<T: RayTracer> Renderer<T> {
    pub fn new(scene: BuiltScene<T>, config: RenderConfig) -> Self {
        Self { scene, config }
    }

    pub fn render_pixel(&self, x: u32, y: u32, camera: &Camera) -> Rgb<f32> {
        let mut result = Rgba::BLACK;

        for _ in 0..self.config.rays_per_pixel {
            let ray = camera.generate_ray(x, y, (fastrand::f32(), fastrand::f32()));

            result += self.trace(ray, AIR_REFRACT, 0) / self.config.rays_per_pixel as f32;
        }

        result.into()
    }

    fn trace(&self, ray: Ray, refraction: f32, depth: u32) -> Rgba {
        if depth > self.config.max_depth {
            // TODO: save this somewhere
            return Rgba::BLACK;
        }

        if let Some(hit) = self.scene.raytracer.intersect(ray) {
            let mut color = Rgba::BLACK;
            let geometry: &Geometry = self
                .scene
                .get_geometry(hit.geometry_id)
                .expect("Error getting geometry");
            let material = &geometry.material;

            let ray_dir = ray.direction;
            let normal = hit.normal;
            let hit_pos = hit.hit_point;
            let u = hit.u;
            let v = hit.v;
            let triangle_id = hit.triangle_id;

            let (diff, emissive) = self.scene.sample_color(geometry, triangle_id, u, v);

            // lighting
            color += self.direct_lighting(hit_pos, normal, &material, diff);

            // diffuse and specular
            if self.config.random_light_transport {
                color += self.random_reflect_refract_scatter(
                    hit_pos, ray_dir, normal, diff, refraction, material, depth,
                );
            } else {
                color += self.reflect_refract_scatter(
                    hit_pos, ray_dir, normal, diff, refraction, material, depth,
                );
            }

            // emissive
            color += emissive;

            color
        } else {
            Rgba::BLACK
        }
    }

    pub fn direct_lighting(
        &self,
        hit_pos: Vec3,
        normal: Vec3,
        material: &Material,
        diffuse: Rgba,
    ) -> Rgba {
        let mut color = Rgba::BLACK;

        let lights = &self.scene.lights;

        if self.config.compare_all_lights {
            // loop over all light sources
            for light in lights.iter() {
                color += self.handle_light(light, hit_pos, normal, material, diffuse);
            }
        } else if let Some(light) = fastrand::choice(lights) {
            color += self.handle_light(light, hit_pos, normal, material, diffuse);
            color *= lights.len() as f32;
        }

        color
    }

    pub fn reflect(
        &self,
        hit: Vec3,
        incident_dir: Vec3,
        normal: Vec3,
        material: &Material,
        depth: u32,
        n1: f32,
        reflect: f32,
    ) -> Rgba {
        let mut color = Rgba::BLACK;

        if reflect > 0.0 && material.reflectivity > 0.0 {
            let refdir = incident_dir.reflect(normal);

            let mut offset = EPSILON * normal;
            if refdir.dot(normal) < 0.0 {
                offset *= -1.0;
            }
            let origin = hit + offset;

            let refraction = n1; // medium did not change

            let reflection_ray = Ray::new(origin, refdir);

            color += self.trace(reflection_ray, refraction, depth + 1);

            color = material.specular * color * reflect; // no need to multiply by reflectivity of the material, fresnel already takes it into account
        }

        color
    }

    pub fn refract(
        &self,
        hit: Vec3,
        incident_dir: Vec3,
        normal: Vec3,
        material: &Material,
        depth: u32,
        n1: f32,
        n2: f32,
        refract: f32,
    ) -> Rgba {
        let mut color = Rgba::BLACK;

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

                let refraction_ray = Ray::new(origin, refract_dir);

                color += self.trace(refraction_ray, refraction, depth + 1);

                color = material.transmission * color * material.transparency * refract;
                // here I use transparency to imply some of the light is lost/absorbed when refracting
                // this is technically not needed and not correct, but beer's law is not yet implemented so this will have to do for now
            }
        }

        color
    }

    // WARN: limited to the first ever hit
    // WARN: exponentially slow, each hit causes N other hits
    // (which is why I stopped it in the first hit)
    pub fn loop_scatter(
        &self,
        hit: Vec3,
        normal: Vec3,
        material: &Material,
        depth: u32,
        n1: f32,
        refract: f32,
        diff: Rgba,
    ) -> Rgba {
        let mut color = Rgba::BLACK;

        if depth == 0 && material.transparency <= 0.0 {
            // takes what's left after reflection
            let diffuse_weight = refract * self.config.diffuse_strength;

            if diffuse_weight > 0.0 {
                for _ in 0..self.config.num_scatter {
                    let scatter_dir = sample_cos_hemisphere(normal);
                    let offset = EPSILON * normal;
                    let origin = hit + offset;
                    let scatter_ray = Ray::new(origin, scatter_dir);
                    let scattered_color = self.trace(scatter_ray, n1, depth + 1);
                    let cos_theta = scatter_dir.dot(normal).max(0.0);

                    color += scattered_color
                        * diff
                        * diffuse_weight
                        * cos_theta
                        * (1.0 / self.config.num_scatter as f32);
                }
            }
        }

        color
    }

    // randomly decides if ray should scatter or not
    // only scatters once
    pub fn scatter_rand(
        &self,
        hit: Vec3,
        normal: Vec3,
        material: &Material,
        depth: u32,
        n1: f32,
        refract: f32,
        diff: Rgba,
    ) -> Rgba {
        let mut color = Rgba::BLACK;

        // TODO: check if diff != 0 before starting?

        if material.transparency <= 0.0 {
            // randomly choose if we scatter
            if fastrand::f32() <= self.config.scatter_probability {
                // takes what's left after reflection
                let diffuse_weight = refract * self.config.diffuse_strength;

                if diffuse_weight > 0.0 {
                    let scatter_dir = sample_cos_hemisphere(normal);
                    let offset = EPSILON * normal;
                    let origin = hit + offset;
                    let scatter_ray = Ray::new(origin, scatter_dir);
                    let scattered_color = self.trace(scatter_ray, n1, depth + 1);
                    let cos_theta = scatter_dir.dot(normal).max(0.0);

                    color += scattered_color
                        * diff
                        * diffuse_weight
                        * cos_theta
                        * (1.0 / self.config.scatter_probability);
                }
            }
        }

        color
    }

    pub fn reflect_refract_scatter(
        &self,
        hit: Vec3,
        incident_dir: Vec3,
        normal: Vec3,
        diff: Rgba,
        ray_eta: f32,
        material: &Material,
        depth: u32,
    ) -> Rgba {
        // n1 is refraction being left
        // n2 is refraction being entered

        let n1: f32;
        let n2: f32;

        // WARNING nao tenho como saber se o raio esta a sair ou entrar no material,
        // nem qual o indice de refracao que devo usar ao sair, entao tive de fazer esta manhosice
        if ray_eta == AIR_REFRACT {
            // raytracer is coming from outside, entering
            n1 = AIR_REFRACT;
            n2 = material.refraction;
        } else {
            // raytracer is leaving
            n1 = material.refraction;
            n2 = AIR_REFRACT;
        }

        let reflect = compute_reflection_coeff(incident_dir, normal, n1, n2, material.reflectivity);
        let refract = 1.0 - reflect;

        // reflection, for materials with reflectivity
        let mut color = self.reflect(hit, incident_dir, normal, material, depth, n1, reflect);

        // refraction, for materials with transparency
        color += self.refract(hit, incident_dir, normal, material, depth, n1, n2, refract);

        // scattering
        // uses what's left after reflection (like refraction), but assumes the material does not refract
        if self.config.use_random_scatter {
            color += self.scatter_rand(hit, normal, material, depth, n1, refract, diff);
        } else {
            color += self.loop_scatter(hit, normal, material, depth, n1, refract, diff);
        }

        color
    }

    // light can reflect, refract or scatter, but only one at a time
    pub fn random_reflect_refract_scatter(
        &self,
        hit: Vec3,
        incident_dir: Vec3,
        normal: Vec3,
        diff: Rgba,
        ray_eta: f32,
        material: &Material,
        depth: u32,
    ) -> Rgba {
        // n1 is refraction being left
        // n2 is refraction being entered

        let n1: f32;
        let n2: f32;

        // WARNING nao tenho como saber se o raio esta a sair ou entrar no material,
        // nem qual o indice de refracao que devo usar ao sair, entao tive de fazer esta manhosice
        if ray_eta == AIR_REFRACT {
            // raytracer is coming from outside, entering
            n1 = AIR_REFRACT;
            n2 = material.refraction;
        } else {
            // raytracer is leaving
            n1 = material.refraction;
            n2 = AIR_REFRACT;
        }

        let reflect = compute_reflection_coeff(incident_dir, normal, n1, n2, material.reflectivity);
        let refract = 1.0 - reflect;

        let rand = fastrand::f32();
        const NUM_CHOICES: f32 = 3.0;

        if rand < 1.0 / NUM_CHOICES {
            // reflection, for materials with reflectivity
            self.reflect(hit, incident_dir, normal, material, depth, n1, reflect) * NUM_CHOICES
        } else if rand < 2.0 / NUM_CHOICES {
            // refraction, for materials with transparency
            self.refract(hit, incident_dir, normal, material, depth, n1, n2, refract) * NUM_CHOICES
        } else {
            // scatter
            // scattering
            // uses what's left after reflection (like refraction), but assumes the material does not refract
            // no loop scattering for now
            self.scatter_rand(hit, normal, material, depth, n1, refract, diff)
        }
    }

    fn handle_light(
        &self,
        light: &Light,
        hit_pos: Vec3,
        normal: Vec3,
        material: &Material,
        diffuse: Rgba,
    ) -> Rgba {
        match &light.light_type {
            LightType::Ambient => Self::handle_ambient_light(light, material),
            LightType::Point(light_pos) => {
                self.handle_point_light(light, diffuse, hit_pos, normal, *light_pos)
            }
            LightType::AreaQuad(square) => {
                self.handle_square_light(light, diffuse, square, hit_pos, normal)
            }
        }
    }

    fn handle_ambient_light(light: &Light, material: &Material) -> Rgba {
        light.color * material.color
    }

    fn handle_point_light(
        &self,
        light: &Light,
        diffuse: Rgba,
        hit_pos: Vec3,
        normal: Vec3,
        light_pos: Vec3,
    ) -> Rgba {
        // if material.diffuse.red > 0.0 || material.diffuse.green > 0.0 || material.diffuse.blue > 0.0
        // {
        // compiler please take care of this
        let distance_to_light = (light_pos - hit_pos).length();
        let dir_to_light = (light_pos - hit_pos).normalize();

        let light_cos = dir_to_light.dot(normal);
        if light_cos > 0.0 {
            // make a raytracer to the light source to check if there is a clear path from the hit position to the light
            // if there is, add light contribution

            let mut offset = EPSILON * normal;
            if dir_to_light.dot(normal) < 0.0 {
                offset *= -1.0;
            }
            let shadow_ray_origin = hit_pos + offset;

            let shadow_ray = Ray::new_with_max_distance(
                shadow_ray_origin,
                dir_to_light,
                distance_to_light - EPSILON,
            );

            // we have a direct path to the light, can add direct illumination
            if let None = self.scene.raytracer.intersect(shadow_ray) {
                let color = light.color * diffuse * light_cos;

                return color;
            }
        }
        // }

        Rgba::BLACK
    }

    // randomly select N points on the light and make them act as individual point lights
    fn handle_square_light(
        &self,
        light: &Light,
        diff: Rgba,
        square: &LightQuad,
        hit_pos: Vec3,
        normal: Vec3,
    ) -> Rgba {
        // if material.diffuse.red > 0.0 || material.diffuse.green > 0.0 || material.diffuse.blue > 0.0
        // {
        let mut color = Rgba::BLACK;
        for _ in 0..self.config.num_area_light_tests {
            let u = fastrand::f32();
            let v = fastrand::f32();

            let light_pos = square.bottom_left + (u * square.u_vec) + (v * square.v_vec);

            // all the logic here is copied from point lights except NUM_AREA_LIGHT_TESTS

            let distance_to_light = (light_pos - hit_pos).length();
            let dir_to_light = (light_pos - hit_pos).normalize();

            let light_cos = dir_to_light.dot(normal);
            let light_coef = light_cos / self.config.num_area_light_tests as f32;
            if light_cos > 0.0 {
                // make a raytracer to the light source to check if there is a clear path from the hit position to the light
                // if there is, add light contribution

                let mut offset = EPSILON * normal;
                if dir_to_light.dot(normal) < 0.0 {
                    offset *= -1.0;
                }
                let shadow_ray_origin = hit_pos + offset;

                let shadow_ray = Ray::new_with_max_distance(
                    shadow_ray_origin,
                    dir_to_light,
                    distance_to_light - EPSILON,
                );

                // we have a direct path to the light, can add direct illumination
                if let None = self.scene.raytracer.intersect(shadow_ray) {
                    color += light.color * diff * light_coef;
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
