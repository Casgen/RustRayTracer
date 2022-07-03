use std::rc::Rc;
use glm::{dot, Vec3, vec3};
use crate::Lambertian;
use crate::math::ray::Ray;
use crate::model::material::Material;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    
    pub t: f32,
    pub material: Rc<dyn Material>,
    pub frontFace: bool
}

impl HitRecord {
    
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        }
    }
    
    pub fn SetFaceNormal(&mut self, ray: &Ray, outwardNormal: Vec3) {
        
        self.frontFace = dot(ray.direction, outwardNormal) < 0.0;
        if self.frontFace { self.normal = outwardNormal } else {self.normal = -outwardNormal};
    }
    
}

impl Default for HitRecord {
    fn default() -> Self {
        return Self {
            point:  vec3(0.0,0.0,0.0),
            normal: vec3(0.0,0.0,0.0),
            t: 0.0,
            material: Rc::new(Lambertian::default()),
            frontFace: false,
        }
    }
}