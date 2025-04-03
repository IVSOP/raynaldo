#![allow(dead_code)] // many presets available, not all used

use glam::Vec3;

// terrible name for both the enum itself and the things inside
#[derive(Debug)]
pub enum RayTransportConfig {
    // ray can EITHER reflect, refract or scatter
    MonteCarloSingle,
    // scatter based on probability
    MonteCarloScatter(f32), // probability, [0, 1]
    // fixed number of scatters
    LoopScatter(u32), // number of scatters, > 0
}

#[derive(Debug)]
pub struct RenderConfig {
    pub max_depth: u32,
    pub compare_all_lights: bool,
    pub num_area_light_tests: u32,
    pub rays_per_pixel: u32,
    pub diffuse_strength: f32,
    pub ray_transport: RayTransportConfig,
}

impl RenderConfig {
    pub const FASTEST: Self = Self {
        max_depth: 4,
        compare_all_lights: false,
        num_area_light_tests: 1,
        rays_per_pixel: 10,
        diffuse_strength: 1.0,
        ray_transport: RayTransportConfig::MonteCarloSingle,
    };

    pub const BALANCED: Self = Self {
        max_depth: 4,
        compare_all_lights: false,
        num_area_light_tests: 4,
        rays_per_pixel: 50,
        diffuse_strength: 1.0,
        ray_transport: RayTransportConfig::MonteCarloScatter(0.2),
    };

    pub const BALANCED_RANDOM_TRANSPORT: Self = Self {
        max_depth: 4,
        compare_all_lights: false,
        num_area_light_tests: 1,
        rays_per_pixel: 50,
        diffuse_strength: 1.0,
        ray_transport: RayTransportConfig::MonteCarloSingle,
    };

    // uses a lot of monte carlo approaches so rays per pixel etc need to be high
    pub const SLOWEST_RAND: Self = Self {
        max_depth: 5,
        compare_all_lights: false,
        num_area_light_tests: 50,
        rays_per_pixel: 100,
        diffuse_strength: 1.0,
        ray_transport: RayTransportConfig::MonteCarloScatter(0.5),
    };

    // does not use a lot of monte carlo approaches
    pub const SLOWEST: Self = Self {
        max_depth: 5,
        compare_all_lights: true,
        num_area_light_tests: 4,
        rays_per_pixel: 20,
        diffuse_strength: 1.0,
        ray_transport: RayTransportConfig::LoopScatter(5),
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

    pub const BALANCED: Self = Self {
        w: 1920, // 2560;
        h: 1080, // 1440;
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
