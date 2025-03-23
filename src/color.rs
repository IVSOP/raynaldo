use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Rgba {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const NONE: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Rgba {
        Self::new(r, g, b, 1.0)
    }
}

impl Add for Rgba {
    type Output = Rgba;

    fn add(self, rhs: Self) -> Self::Output {
        Rgba {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Div<f32> for Rgba {
    type Output = Rgba;

    fn div(self, rhs: f32) -> Self::Output {
        Rgba {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl AddAssign for Rgba {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Mul for Rgba {
    type Output = Rgba;

    fn mul(self, rhs: Self) -> Self::Output {
        Rgba {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Mul<f32> for Rgba {
    type Output = Rgba;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgba {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl MulAssign<f32> for Rgba {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl Into<image::Rgba<f32>> for Rgba {
    fn into(self) -> image::Rgba<f32> {
        image::Rgba([self.r, self.g, self.b, self.a])
    }
}

impl Into<image::Rgb<f32>> for Rgba {
    fn into(self) -> image::Rgb<f32> {
        image::Rgb([self.r, self.g, self.b])
    }
}

impl Into<Rgba> for image::Rgba<f32> {
    fn into(self) -> Rgba {
        let [r, g, b, a] = self.0;
        Rgba::new(r, g, b, a)
    }
}
