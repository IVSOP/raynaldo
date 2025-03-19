#![allow(dead_code)]
use super::*;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: LinearRgba,
    // pub emissive: LinearRgba,
    // pub diffuse: LinearRgba,
    pub specular: LinearRgba,
    pub transmission: LinearRgba,
    pub refraction: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub texture: Texture,
    pub emissive: Texture,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: LinearRgba::RED,
            // diffuse: LinearRgba::RED,
            specular: LinearRgba::RED,
            transmission: LinearRgba::RED,
            refraction: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            texture: Texture::Solid(LinearRgba::RED),
            emissive: Texture::Solid(LinearRgba::NONE),
        }
    }
}

impl Material {
    pub const WHITE_MATERIAL: Self = Self {
        color: LinearRgba::rgb(0.9, 0.9, 0.9),
        texture: Texture::Solid(LinearRgba::rgb(0.4, 0.4, 0.4)),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };

    pub const RED_MATERIAL: Self = Self {
        color: LinearRgba::rgb(0.9, 0.0, 0.0),
        texture: Texture::Solid(LinearRgba::rgb(0.4, 0.0, 0.0)),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };
    pub const GREEN_MATERIAL: Self = Self {
        color: LinearRgba::rgb(0.0, 0.9, 0.0),
        texture: Texture::Solid(LinearRgba::rgb(0.0, 0.2, 0.0)),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };
    pub const BLUE_MATERIAL: Self = Self {
        color: LinearRgba::rgb(0.0, 0.0, 0.9),
        texture: Texture::Solid(LinearRgba::rgb(0.0, 0.0, 0.4)),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };
    pub const ORANGE_MATERIAL: Self = Self {
        color: LinearRgba::rgb(0.99, 0.65, 0.0),
        texture: Texture::Solid(LinearRgba::rgb(0.37, 0.24, 0.0)),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };
    pub const MIRROR_MATERIAL: Self = Self {
        color: LinearRgba::BLACK,
        texture: Texture::Solid(LinearRgba::BLACK),
        specular: LinearRgba::rgb(0.9, 0.9, 0.9),
        transmission: LinearRgba::BLACK,
        refraction: 1.5,
        reflectivity: 1.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };
    pub const GLASS_MATERIAL: Self = Self {
        color: LinearRgba::WHITE,
        texture: Texture::Solid(LinearRgba::BLACK),
        specular: LinearRgba::rgb(1.0, 1.0, 1.0),
        transmission: LinearRgba::rgb(0.9, 0.9, 0.9),
        refraction: 1.125,
        reflectivity: 0.1, // try 0.01
        transparency: 1.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };
    pub const UV_MATERIAL: Self = Self {
        color: LinearRgba::rgb(1.0, 1.0, 1.0),
        texture: Texture::Image(0),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::NONE),
    };

    pub const EMISSIVE_MATERIAL: Self = Self {
        color: LinearRgba::rgb(1.0, 1.0, 1.0),
        texture: Texture::Image(0),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(LinearRgba::RED),
    };

    pub const CUBEMAP_MATERIAL: Self = Self {
        color: LinearRgba::BLACK,
        texture: Texture::Image(0),
        specular: LinearRgba::BLACK,
        transmission: LinearRgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Image(1),
    };
}
