use std::ops::Deref;
use std::rc::Rc;
use glm::{dot, sqrt, Vec3, vec3};
use crate::Lambertian;
use crate::math::mathUtils::vectorLengthSquared;
use crate::math::ray::Ray;
use crate::model::material::Material;
use crate::model::hitRecord::HitRecord;
use crate::model::hittableObject::HittableObject;

pub struct Sphere {
    radius: f32,
    center: Vec3,
    material: Rc<dyn Material>
}


impl HittableObject for Sphere {
    fn testIntersection(&self, ray: &Ray, tMin: f32, tMax: f32, hitRecord: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        
        let a: f32 = vectorLengthSquared(&ray.direction);
        let half_b: f32 = dot(oc,ray.direction);
        let c: f32 = vectorLengthSquared(&oc) - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            return false;
        }
        
        let sqrtd: f32 = sqrt(discriminant);
        
        let mut root: f32 = (-half_b - sqrtd) / a;
        
        if root < tMin || tMax < root {
            
            root = (-half_b + sqrtd) / a;
            if root < tMin || tMax < root {
                return false;
            }
        }
        
        hitRecord.point = ray.at(root);
        hitRecord.normal = vec3(0.0,0.0,0.0);
        hitRecord.t = root;
        hitRecord.material = Rc::clone(&self.material);
        
        let outwardNormal: Vec3 = (hitRecord.point - self.center) / self.radius;
        
        hitRecord.SetFaceNormal(ray,outwardNormal);
        
        return true;
    }
}

impl Sphere {
    
    /* Creates a new Sphere with a given radius */
    pub fn new(r: f32, position: Vec3, mat: Rc<dyn Material>) -> Self {
        return Sphere {
            radius: r,
            center: position,
            material: mat,
        }
    }
    
    pub fn getRadius(&self) -> f32 {
        return self.radius;
    }
    
    pub fn setRadius(&mut self, radius: f32) {
        self.radius = radius;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        return Self {
            radius: 1.0,
            center: vec3(0.0,0.0,0.0),
            material: Rc::new(Lambertian::default()),
        }
    }
}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        return Sphere {
            radius: self.radius,
            center: self.center.clone(),
            material: self.material.clone()
        }
    }
}

