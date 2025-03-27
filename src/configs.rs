#![allow(dead_code)] // many presets available, not all used

use glam::Vec3;

#[derive(Debug)]
pub struct RenderConfig {
    pub max_depth: u32,
    pub compare_all_lights: bool,
    pub num_area_light_tests: u32,
    pub rays_per_pixel: u32,
    pub num_scatter: u32,
    pub diffuse_strength: f32,
    pub scatter_probability: f32,
    pub use_random_scatter: bool,
}

impl RenderConfig {
    pub const FASTEST: Self = Self {
        max_depth: 4,
        compare_all_lights: false,
        num_area_light_tests: 1,
        rays_per_pixel: 10,
        num_scatter: 1,
        diffuse_strength: 1.0,
        scatter_probability: 0.0,
        use_random_scatter: false,
    };

    pub const BALANCED: Self = Self {
        max_depth: 4,
        compare_all_lights: false,
        num_area_light_tests: 4,
        rays_per_pixel: 50,
        num_scatter: 0,
        diffuse_strength: 1.0,
        scatter_probability: 0.2,
        use_random_scatter: true,
    };

    // uses a lot of monte carlo approaches so rays per pixel etc need to be high
    pub const SLOWEST_RAND: Self = Self {
        max_depth: 5,
        compare_all_lights: false,
        num_area_light_tests: 10,
        rays_per_pixel: 100,
        num_scatter: 0,
        diffuse_strength: 1.0,
        scatter_probability: 0.5,
        use_random_scatter: true,
    };

    // does not use a lot of monte carlo approaches
    pub const SLOWEST: Self = Self {
        max_depth: 5,
        compare_all_lights: true,
        num_area_light_tests: 4,
        rays_per_pixel: 20,
        num_scatter: 5,
        diffuse_strength: 1.0,
        scatter_probability: 0.0,
        use_random_scatter: false,
    };
}

#[derive(Debug)]
pub struct CamConfig {
    pub w: u32,
    pub h: u32,
    pub pos: Vec3,
    pub lookat: Vec3,
    pub fov: f32,
}

impl CamConfig {
    pub const FAST: Self = Self {
        w: 640,
        h: 640,
        pos: Vec3::new(280.0, 265.0, -500.0),
        lookat: Vec3::new(280.0, 260.0, 0.0),
        fov: 60.0_f32.to_radians(),
    };

    pub const SLOW: Self = Self {
        w: 1920, // 2560;
        h: 1080, // 1440;
        pos: Vec3::new(280.0, 265.0, -480.0),
        lookat: Vec3::new(280.0, 260.0, 0.0),
        fov: 65.0_f32.to_radians(),
    };
}
