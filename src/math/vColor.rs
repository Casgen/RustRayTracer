use std::ops::{Add, AddAssign, Div, Mul};
use glm::{clamp, Vec3, vec3, vec4, Vec4};
use sfml::graphics::Color;

pub struct VColor {
    pub color: Vec4,
}

impl VColor {
    pub fn new_sc(scalar: f32) -> Self {
        return Self {
            color: vec4(
                clamp(scalar, 0.0, 1.0),
                clamp(scalar, 0.0, 1.0),
                clamp(scalar, 0.0, 1.0),
                1.0)
        };
    }
    
    pub fn new_vec4(vec: &Vec4) -> Self {
        return Self {
            color: vec.clone()
        }
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        return Self {
            color: vec4(clamp(r, 0.0, 1.0),
                        clamp(g, 0.0, 1.0),
                        clamp(b, 0.0, 1.0),
                        1.0)
        };
    }

    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        return Self {
            color: vec4(clamp(r, 0.0, 1.0),
                        clamp(g, 0.0, 1.0),
                        clamp(b, 0.0, 1.0),
                        clamp(a, 0.0, 1.0))
        };
    }

    pub fn getColorRGB(&self) -> Vec3 {
        return vec3(self.color.x, self.color.y, self.color.z);
    }

    pub fn asSFColor(&self) -> Color {
        return Color::new_rgba(
            (self.color.x * 255.0) as u8,
            (self.color.y * 255.0) as u8,
             (self.color.z * 255.0) as u8,
              (self.color.w * 255.0) as u8,
        );
    }
}

//-----------Operators--------------
impl Add<VColor> for VColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        return VColor{ color: self.color + rhs.color }
    }
}

impl Div<VColor> for VColor {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        return VColor { color: self.color / rhs.color}
    }
}

impl Mul<VColor> for VColor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        return VColor { color: self.color * rhs.color }
    }
}

impl AddAssign<VColor> for VColor {
    fn add_assign(&mut self, rhs: VColor) {
        self.color = self.color + rhs.color;
    }
}

impl Mul<f32> for VColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        return VColor { color: self.color * rhs }
    }
}

impl Div<i32> for VColor {
    type Output = VColor;

    fn div(self, rhs: i32) -> Self::Output {
        return VColor { color: self.color / rhs as f32}
    }
}

//-----------Operators--------------


impl Default for VColor {
    fn default() -> Self {
        return Self {
            color: vec4(0.0, 0.0, 0.0, 0.0)
        };
    }
}

impl Clone for VColor {
    fn clone(&self) -> Self {
        return VColor{
            color: self.color
        }
    }
}