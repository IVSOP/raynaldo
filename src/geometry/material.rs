use crate::color::Rgba;
use crate::geometry::Texture;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Rgba, // ambient / albedo
    // pub emissive: Rgba,
    // pub diffuse: Rgba,
    pub specular: Rgba,     // reflections
    pub transmission: Rgba, // refractions
    pub refraction: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub texture: Texture,  // diffuse
    pub emissive: Texture, // emissive
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Rgba::RED,
            // diffuse: Rgba::RED,
            specular: Rgba::RED,
            transmission: Rgba::RED,
            refraction: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            texture: Texture::Solid(Rgba::RED),
            emissive: Texture::Solid(Rgba::NONE),
        }
    }
}

impl Material {
    pub const WHITE_MATERIAL: Self = Self {
        color: Rgba::rgb(0.9, 0.9, 0.9),
        texture: Texture::Solid(Rgba::rgb(0.4, 0.4, 0.4)),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::NONE),
    };

    pub const RED_MATERIAL: Self = Self {
        color: Rgba::rgb(0.9, 0.0, 0.0),
        texture: Texture::Solid(Rgba::rgb(0.4, 0.0, 0.0)),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::NONE),
    };
    pub const GREEN_MATERIAL: Self = Self {
        color: Rgba::rgb(0.0, 0.9, 0.0),
        texture: Texture::Solid(Rgba::rgb(0.0, 0.2, 0.0)),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::NONE),
    };
    pub const BLUE_MATERIAL: Self = Self {
        color: Rgba::rgb(0.0, 0.0, 0.9),
        texture: Texture::Solid(Rgba::rgb(0.0, 0.0, 0.4)),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::NONE),
    };
    pub const ORANGE_MATERIAL: Self = Self {
        color: Rgba::rgb(0.99, 0.65, 0.0),
        texture: Texture::Solid(Rgba::rgb(0.37, 0.24, 0.0)),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::NONE),
    };
    pub const MIRROR_MATERIAL: Self = Self {
        color: Rgba::BLACK,
        texture: Texture::Solid(Rgba::BLACK),
        specular: Rgba::rgb(0.9, 0.9, 0.9),
        transmission: Rgba::BLACK,
        refraction: 1.5,
        reflectivity: 1.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::NONE),
    };
    pub const GLASS_MATERIAL: Self = Self {
        color: Rgba::WHITE,
        texture: Texture::Solid(Rgba::BLACK),
        specular: Rgba::rgb(1.0, 1.0, 1.0),
        transmission: Rgba::rgb(0.9, 0.9, 0.9),
        refraction: 1.125,
        reflectivity: 0.1, // try 0.01
        transparency: 1.0,
        emissive: Texture::Solid(Rgba::NONE),
    };
    pub const UV_MATERIAL: Self = Self {
        color: Rgba::rgb(1.0, 1.0, 1.0),
        texture: Texture::Image(0),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::NONE),
    };

    pub const EMISSIVE_MATERIAL: Self = Self {
        color: Rgba::rgb(1.0, 1.0, 1.0),
        texture: Texture::Image(0),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Solid(Rgba::RED),
    };

    pub const CUBEMAP_MATERIAL: Self = Self {
        color: Rgba::BLACK,
        texture: Texture::Solid(Rgba::BLACK),
        specular: Rgba::BLACK,
        transmission: Rgba::BLACK,
        refraction: 1.0,
        reflectivity: 0.0,
        transparency: 0.0,
        emissive: Texture::Image(1),
    };
}
