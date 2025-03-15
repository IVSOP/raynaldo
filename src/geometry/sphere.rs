use embree4_rs::geometry::UserGeometryImpl;
use glam::Vec3;
use std::f32::consts::PI;
use super::*; // prefiro que a struct esteja no mod.rs fica mais facil de importar

impl UserGeometryImpl for Sphere {
    fn bounds(&self) -> embree4_sys::RTCBounds {
        println!("min {} max {} center {} radius {}", self.center - self.radius, self.center + self.radius, self.center, self.radius);
        embree4_sys::RTCBounds {
            lower_x: self.center.x - self.radius,
            lower_y: self.center.y - self.radius,
            lower_z: self.center.z - self.radius,
            align0: 0.0,
            upper_x: self.center.x + self.radius,
            upper_y: self.center.y + self.radius,
            upper_z: self.center.z + self.radius,
            align1: 0.0,
            // ..Default::default()
        }
    }

    fn intersect(
        &self,
        geom_id: u32,
        prim_id: u32,
        ctx: &embree4_sys::RTCRayQueryContext,
        ray_hit: &mut embree4_sys::RTCRayHit,
    ) {
        // let origin = Vec3::new(ray_hit.ray.org_x, ray_hit.ray.org_y, ray_hit.ray.org_z);
        // let dir = Vec3::new(ray_hit.ray.dir_x, ray_hit.ray.dir_y, ray_hit.ray.dir_z);
        // let hit_pos = origin + dir * ray_hit.ray.tfar;
        // println!("hit bb at {}", hit_pos);
        // println!("hit");

        let origin = Vec3::new(ray_hit.ray.org_x, ray_hit.ray.org_y, ray_hit.ray.org_z);
        let dir = Vec3::new(ray_hit.ray.dir_x, ray_hit.ray.dir_y, ray_hit.ray.dir_z);
        let to_center = origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * to_center.dot(dir);
        let c = to_center.dot(to_center) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        // If we have no intersection, we can exit early
        if discriminant < 0.0 {
            return;
        }

        // let sqrt_d = discriminant.sqrt();
        // let mut t = (-b - sqrt_d) / (2.0 * a);
        // if t < ray_hit.ray.tnear || t > ray_hit.ray.tfar {
        //     t = (-b + sqrt_d) / (2.0 * a);
        //     if t < ray_hit.ray.tnear || t > ray_hit.ray.tfar {
        //         return;
        //     }
        // }


        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let t = t1.min(t2);
        ray_hit.ray.tfar = t;

        let n = (origin + t * dir - self.center).normalize();
        ray_hit.hit.Ng_x = n.x;
        ray_hit.hit.Ng_y = n.y;
        ray_hit.hit.Ng_z = n.z;

        // calculate uv coordinates
        let p = origin + t * dir;
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();

        let u = 1.0 - (phi + PI) / (2.0 * PI);
        ray_hit.hit.u = u;

        let v = (theta + PI / 2.0) / PI;
        ray_hit.hit.v = v;

        ray_hit.hit.instID = ctx.instID;
        ray_hit.hit.geomID = geom_id;
        ray_hit.hit.primID = prim_id;
    }
}
