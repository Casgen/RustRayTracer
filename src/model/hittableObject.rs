use crate::math::ray::Ray;
use crate::model::hitRecord::HitRecord;


pub trait HittableObject {
    
    fn testIntersection(&self, ray: &Ray, tMin: f32, tMax: f32, hitRecord: &mut HitRecord) -> bool;
}

