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
    pub min_depth: u32,
    /// inverse of the probability of still going deeper when if depth >= min_depth
    pub over_depth_prob: f32,
    pub compare_all_lights: bool,
    pub num_area_light_tests: u32,
    pub rays_per_pixel: u32,
    pub diffuse_strength: f32,
    pub ray_transport: RayTransportConfig,
}

impl RenderConfig {
    pub const fn new(
        min_depth: u32,
        over_depth_prob: f32,
        compare_all_lights: bool,
        num_area_light_tests: u32,
        rays_per_pixel: u32,
        diffuse_strength: f32,
        ray_transport: RayTransportConfig,
    ) -> Self {
        if over_depth_prob == 0.0 {
            panic!("Acho que dividir por 0 nao vai correr muito bem");
        }
        Self {
            min_depth,
            over_depth_prob,
            compare_all_lights,
            num_area_light_tests,
            rays_per_pixel,
            diffuse_strength,
            ray_transport,
        }
    }

    pub const fn fastest() -> Self {
        Self::new(
            4,
            0.1,
            false,
            1,
            10,
            1.0,
            RayTransportConfig::MonteCarloSingle,
        )
    }

    pub const fn balanced() -> Self {
        Self::new(
            4,
            0.5,
            false,
            4,
            50,
            1.0,
            RayTransportConfig::MonteCarloScatter(0.2),
        )
    }

    pub const fn balanced_random_transport() -> Self {
        Self::new(
            4,
            0.5,
            false,
            1,
            50,
            1.0,
            RayTransportConfig::MonteCarloSingle,
        )
    }

    // uses a lot of monte carlo approaches so rays per pixel etc need to be high
    pub const fn slowest_rand() -> Self {
        Self::new(
            5,
            0.8,
            false,
            50,
            100,
            1.0,
            RayTransportConfig::MonteCarloScatter(0.5),
        )
    }

    // does not use a lot of monte carlo approaches
    pub const fn slowest() -> Self {
        Self::new(5, 0.8, true, 4, 20, 1.0, RayTransportConfig::LoopScatter(5))
    }
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
