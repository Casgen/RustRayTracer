use std::ops::Mul;
use glm::{Vec3, vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        return Self {
            origin: orig,
            direction: dir,
        };
    }

    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + self.direction * t;
    }
}

impl Default for Ray {
    fn default() -> Self {
        return Self {
            origin: vec3(0.0,0.0,0.0),
            direction: vec3(0.0,0.0,0.0),
        }
    }
}