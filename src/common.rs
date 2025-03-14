#![allow(dead_code)]

use glam::*;
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
