#![allow(dead_code)]

use glam::{Vec2, Vec3};
use std::ops::RangeBounds;

#[inline]
pub fn randu32(range: impl RangeBounds<u32>) -> u32 {
    fastrand::u32(range)
}

/// NOTE: does not include 1 or -1, is only ]-1, 1[
#[inline]
pub fn randf32_normalized() -> f32 {
    1.0 - (fastrand::f32() * 2.0)
}

// TODO: make this also receive a range??
// NOTE: I have no idea if this will ever include either min or max, but it is somewhere in the
// middle
#[inline]
pub fn randf32_range(min: f32, max: f32) -> f32 {
    debug_assert!(min < max, "min should be less than max");
    min + (max - min) * fastrand::f32()
}

// TODO: test if faster than randomly creating 3x random floats and normalizing the vector? I could generate x, use the remaining valid len to generate y, use remaining len to generate z
#[inline]
pub fn rand_dir() -> Vec3 {
    Vec3::new(
        randf32_normalized(),
        randf32_normalized(),
        randf32_normalized(),
    )
    .normalize()
}

#[inline]
pub fn _rand_dir2() -> Vec2 {
    Vec2::new(randf32_normalized(), randf32_normalized()).normalize()
}

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}

/// use schlick approximation to compute fraction of light specularly reflected at a surface (fresnel)
pub fn compute_reflection_coeff(
    incident_dir: Vec3,
    normal: Vec3,
    n1: f32, // refraction being left
    n2: f32, // refraction being entered
    material_reflectivity: f32,
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

    material_reflectivity + (1.0 - material_reflectivity) * ret
}
