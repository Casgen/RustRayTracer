use glm::{dot, normalize, reflect, Vec3};
use crate::math::ray::Ray;
use crate::math::vColor::VColor;
use crate::model::hitRecord::HitRecord;
use crate::model::material::Material;

pub struct Metal {
    albedo: VColor,
    fuzz: f32
}

impl Metal {
    pub fn new(color: VColor, fuzziness: f32) -> Self {
        return Self {
            albedo: color,
            fuzz: fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hitRecord: &HitRecord, attenuation: &mut VColor, scattered: &mut Ray) -> bool {
        let reflectedRay: Vec3 = reflect(normalize(ray.direction),hitRecord.normal);

        *scattered = Ray::new(hitRecord.point, reflectedRay + hitRecord.normal * self.fuzz);

        *attenuation = self.albedo.clone();
        return dot(scattered.direction,hitRecord.normal) > 0.0;
    }
}