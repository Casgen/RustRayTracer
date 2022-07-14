use crate::math::ray::Ray;
use crate::math::vColor::VColor;
use crate::model::hitRecord::HitRecord;

pub trait Material {
    fn scatter(&self, ray: &Ray, hitRecord: &HitRecord, attenuation: &mut VColor, scattered: &mut Ray) -> bool;
}
