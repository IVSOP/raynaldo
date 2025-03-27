use glam::Vec3;

pub const EPSILON: f32 = 1e-3;
pub const AIR_REFRACT: f32 = 1.00029;

pub struct Consts;

#[cfg(debug_assertions)]
impl Consts {
    pub const W: u32 = 640;
    pub const H: u32 = 640;
    pub const CAM_POS: Vec3 = Vec3::new(280.0, 265.0, -500.0);
    pub const CAM_LOOKAT: Vec3 = Vec3::new(280.0, 260.0, 0.0);
    pub const CAM_FOV: f32 = 60.0_f32.to_radians();
    pub const MAX_DEPTH: u32 = 4;
    pub const COMPARE_ALL_LIGHTS: bool = false;
    pub const NUM_AREA_LIGHT_TESTS: u32 = 1;
    pub const RAYS_PER_PIXEL: u32 = 10;
    pub const NUM_SCATTER: u32 = 1;
    pub const DIFFUSE_STRENGTH: f32 = 1.0; // acho que isto nunca vai ser alterado mas fica aqui
}

#[cfg(not(debug_assertions))]
impl Consts {
    pub const W: u32 = 1920; // 2560;
    pub const H: u32 = 1080; // 1440;
    pub const CAM_POS: Vec3 = Vec3::new(280.0, 265.0, -500.0);
    pub const CAM_LOOKAT: Vec3 = Vec3::new(280.0, 260.0, 0.0);
    pub const CAM_FOV: f32 = 65.0_f32.to_radians();
    pub const MAX_DEPTH: u32 = 5;
    pub const COMPARE_ALL_LIGHTS: bool = true;
    pub const NUM_AREA_LIGHT_TESTS: u32 = 4;
    pub const RAYS_PER_PIXEL: u32 = 20;
    pub const NUM_SCATTER: u32 = 5;
    pub const DIFFUSE_STRENGTH: f32 = 1.0; // acho que isto nunca vai ser alterado mas fica aqui
}
