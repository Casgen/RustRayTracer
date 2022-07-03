use glm::Vec3;
use crate::math::mathUtils::{isNearlyZero_Vec3, isNearlyZero_Vec4, randomUnitVector3};
use crate::math::ray::Ray;
use crate::math::vColor::VColor;
use crate::model::hitRecord::HitRecord;
use crate::model::material::Material;

pub struct Lambertian {
    albedo: VColor
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hitRecord: &HitRecord, attenuation: &mut VColor, scattered: &mut Ray) -> bool {
        
        let mut scatterDir: Vec3 = hitRecord.normal + randomUnitVector3();
        
        if isNearlyZero_Vec3(&scatterDir, None) {
            scatterDir = hitRecord.normal;
        }
        
        *scattered = Ray::new(hitRecord.point,scatterDir);
        *attenuation = self.albedo.clone();
        return true;
    }
}

impl Lambertian {
    pub fn new(color: VColor) -> Self {
        return Self {
            albedo: color
        }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        return Self {
            albedo: VColor::new_sc(0.5)
        }
    }
}