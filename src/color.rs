use bevy_math::Vec4;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    color: Vec4,
}

impl Rgba {}

impl Rgba {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const NONE: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            color: Vec4::new(r, g, b, a),
        }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Rgba {
        Self::new(r, g, b, 1.0)
    }
}

impl<T: Into<Rgba>> Add<T> for Rgba {
    type Output = Rgba;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Rgba {
            color: self.color + rhs.color,
        }
    }
}

impl Div<f32> for Rgba {
    type Output = Rgba;

    fn div(self, rhs: f32) -> Self::Output {
        Rgba {
            color: self.color / rhs,
        }
    }
}

impl AddAssign for Rgba {
    fn add_assign(&mut self, rhs: Self) {
        self.color += rhs.color;
    }
}

impl Mul for Rgba {
    type Output = Rgba;

    fn mul(self, rhs: Self) -> Self::Output {
        Rgba {
            color: self.color * rhs.color,
        }
    }
}

impl Mul<f32> for Rgba {
    type Output = Rgba;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgba {
            color: self.color * rhs,
        }
    }
}

impl MulAssign<f32> for Rgba {
    fn mul_assign(&mut self, rhs: f32) {
        self.color *= rhs;
    }
}

impl Into<image::Rgba<f32>> for Rgba {
    fn into(self) -> image::Rgba<f32> {
        let [r, g, b, a] = self.color.into();
        image::Rgba([r, g, b, a])
    }
}

impl Into<image::Rgb<f32>> for Rgba {
    fn into(self) -> image::Rgb<f32> {
        let [r, g, b, _a] = self.color.into();
        image::Rgb([r, g, b])
    }
}

impl Into<Rgba> for image::Rgba<f32> {
    fn into(self) -> Rgba {
        let [r, g, b, a] = self.0;
        Rgba::new(r, g, b, a)
    }
}
