use crate::camera::Camera;
use crate::consts::{AIR_REFRACT, Consts, EPSILON};
use crate::geometry::{BuiltScene, Geometry, Light, LightQuad, LightType, Material};
use crate::raytracer::{Ray, RayTracer};
use bevy_color::{ColorToComponents, LinearRgba};
use bevy_math::Vec3;
use image::{Rgb, Rgb32FImage};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

pub struct Renderer<T: RayTracer> {
    scene: BuiltScene<T>,
}

impl<T: RayTracer + Sync> Renderer<T> {
    pub fn render_par(&self, camera: &Camera) -> Rgb32FImage {
        Rgb32FImage::from_par_fn(camera.w, camera.h, |x, y| self.render_pixel(x, y, camera))
    }

    pub fn render_par_with_progress(
        &self,
        camera: &Camera,
        progress: Arc<AtomicUsize>,
    ) -> Rgb32FImage {
        Rgb32FImage::from_par_fn(camera.w, camera.h, |x, y| {
            let result = self.render_pixel(x, y, camera);
            progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            result
        })
    }
}

impl<T: RayTracer> Renderer<T> {
    pub fn new(scene: BuiltScene<T>) -> Self {
        Self { scene }
    }

    pub fn render_pixel(&self, x: u32, y: u32, camera: &Camera) -> Rgb<f32> {
        let mut result = LinearRgba::BLACK;

        for _ in 0..Consts::RAYS_PER_PIXEL {
            let ray = camera.generate_ray(x, y, (fastrand::f32(), fastrand::f32()));

            result += self.trace(ray, AIR_REFRACT, 0) / Consts::RAYS_PER_PIXEL as f32;
        }

        result.to_f32_array_no_alpha().into()
    }

    fn trace(&self, ray: Ray, refraction: f32, depth: u32) -> LinearRgba {
        if depth > Consts::MAX_DEPTH {
            // TODO: save this somewhere
            return LinearRgba::BLACK;
        }

        if let Some(hit) = self.scene.raytracer.intersect(ray) {
            let mut color = LinearRgba::BLACK;
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

            color += self.direct_lighting(hit_pos, normal, &material, diff);
            color +=
                self.reflect_refract(hit_pos, ray_dir, normal, diff, refraction, material, depth);

            color += emissive;

            color
        } else {
            LinearRgba::BLACK
        }
    }

    pub fn direct_lighting(
        &self,
        hit_pos: Vec3,
        normal: Vec3,
        material: &Material,
        diffuse: LinearRgba,
    ) -> LinearRgba {
        let mut color = LinearRgba::BLACK;

        let lights = &self.scene.lights;

        if Consts::COMPARE_ALL_LIGHTS {
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
    ) -> LinearRgba {
        // n1 is refraction being left
        // n2 is refraction being entered

        let n1: f32;
        let n2: f32;

        // WARNING nao tenho como saber se o raio esta a sair ou entrar no material,
        // nem qual o indice de refracao que devo usar ao sair, entao tive de fazer esta manhosice
        if ray_refraction == AIR_REFRACT {
            // raytracer is coming from outside, entering
            n1 = AIR_REFRACT;
            n2 = material.refraction;
        } else {
            // raytracer is leaving
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

            let reflection_ray = Ray::new(origin, refdir);

            color += self.trace(reflection_ray, refraction, depth + 1);

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

                let refraction_ray = Ray::new(origin, refract_dir);

                color += self.trace(refraction_ray, refraction, depth + 1);

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
                    let scatter_ray = Ray::new(origin, scatter_dir);
                    let scattered_color = self.trace(scatter_ray, n1, depth + 1);
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

    fn handle_light(
        &self,
        light: &Light,
        hit_pos: Vec3,
        normal: Vec3,
        material: &Material,
        diffuse: LinearRgba,
    ) -> LinearRgba {
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

    fn handle_ambient_light(light: &Light, material: &Material) -> LinearRgba {
        LinearRgba::rgb(
            light.color.red * material.color.red,
            light.color.green * material.color.green,
            light.color.blue * material.color.blue,
        )
    }

    fn handle_point_light(
        &self,
        light: &Light,
        diffuse: LinearRgba,
        hit_pos: Vec3,
        normal: Vec3,
        light_pos: Vec3,
    ) -> LinearRgba {
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
                let color = LinearRgba::rgb(
                    light.color.red * diffuse.red,
                    light.color.green * diffuse.green,
                    light.color.blue * diffuse.blue,
                ) * light_cos;

                return color;
            }
        }
        // }

        LinearRgba::BLACK
    }

    // randomly select N points on the light and make them act as individual point lights
    fn handle_square_light(
        &self,
        light: &Light,
        diff: LinearRgba,
        square: &LightQuad,
        hit_pos: Vec3,
        normal: Vec3,
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
            let light_coef = light_cos / Consts::NUM_AREA_LIGHT_TESTS as f32;
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
                    color += LinearRgba::rgb(
                        light.color.red * diff.red,
                        light.color.green * diff.green,
                        light.color.blue * diff.blue,
                    ) * light_coef;
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
