use glm::{dot, normalize, pow, sqrt, Vec3};
use rand::{Rng, thread_rng};
use crate::math::mathUtils::{reflect, refract};
use crate::math::ray::Ray;
use crate::math::vColor::VColor;
use crate::model::hitRecord::HitRecord;
use crate::model::material::Material;

pub struct Dielectric {
    indexOfRefraction: f32,
}

impl Dielectric {
    pub fn new(index: f32) -> Self {
        return Self {
            indexOfRefraction: index
        };
    }

    pub fn reflectance(cos: f32, refIdx: f32) -> f32 {
        let mut r0: f32 = (1.0 - refIdx) / (1.0 + refIdx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * pow(1.0 - cos, 5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hitRecord: &HitRecord, attenuation: &mut VColor, scattered: &mut Ray) -> bool {
        *attenuation = VColor::new_sc(1.0);

        let refractionRatio: f32 = if hitRecord.frontFace {
            1.0 / self.indexOfRefraction
        } else {
            self.indexOfRefraction
        };

        let unitDirection: Vec3 = normalize(ray.direction);

        let cosTheta: f32 = f32::min(dot(unitDirection * -1.0, hitRecord.normal), 1.0);
        let sinTheta: f32 = sqrt(1.0 - cosTheta * cosTheta);

        let cannotRefract: bool = refractionRatio * sinTheta > 1.0;
        let direction: Vec3;

        if cannotRefract || Dielectric::reflectance(cosTheta,refractionRatio) > thread_rng().gen_range(0.0..1.0) {
            direction = reflect(&unitDirection, &hitRecord.normal);
        } else {
            direction = refract(&unitDirection,&hitRecord.normal,&refractionRatio);
        }

        *scattered = Ray::new(hitRecord.point, direction);
        return true;
    }
}

