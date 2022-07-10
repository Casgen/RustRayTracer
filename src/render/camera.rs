use std::f32::consts::PI;
use glm::{cos, cross, IVec4, ivec4, mat4, Mat4, normalize, radians, sin, tan, vec3, Vec3, Vec4, vec4};
use glm::ext::{look_at, perspective};
use crate::math::mathUtils::randomInUnitDisk;
use crate::math::ray::Ray;

pub struct Camera {
    eyePosition: Vec3,
    viewDirection: Vec3,
    upVector: Vec3,
    sideVector: Vec3,

    lensRadius: f32,

    horizontal: Vec3,
    vertical: Vec3,
    lowerLeftCorner: Vec3,

}


impl Camera {
    pub fn new(position: Vec3, lookAt: Vec3, width: i32, height: i32, aperture: f32, focusDist: f32) -> Camera {

        let aspectRatio: f32 = width as f32 / height as f32;
        let theta: f32 = radians(45.0);
        let h: f32 = tan(theta/2.0);

        let vwHeight: f32 = 2.0 * h;
        let vwWidth: f32 = aspectRatio * vwHeight;

        let viewDirection: Vec3 = normalize(position - lookAt);
        let sideVector: Vec3 = normalize(cross(vec3(0.0,0.0,0.1), viewDirection));
        let upVector: Vec3 = cross(viewDirection, sideVector);

        let horizontal: Vec3 = sideVector * vwWidth * focusDist;
        let vertical: Vec3 = upVector * vwHeight * focusDist;

        return Camera {
            horizontal,
            vertical,
            lowerLeftCorner: position - horizontal/2.0 - vertical/2.0 - viewDirection * focusDist,
            viewDirection,
            upVector,
            sideVector,
            eyePosition: position,
            lensRadius: aperture / 2.0,
        };
    }


    pub fn createARay(&self, x: f32, y: f32) -> Ray {

        let rd: Vec3 = randomInUnitDisk() * self.lensRadius;
        let offset: Vec3 = self.upVector * rd.x + self.sideVector * rd.y;

        return Ray::new(self.eyePosition + offset,  self.lowerLeftCorner + self.horizontal * x + self.vertical * y - self.eyePosition - offset);
    }
    
}

impl Default for Camera {
    fn default() -> Self {
        let aspectRatio: f32 = 800.0 as f32 / 600.0 as f32;
        let theta: f32 = radians(45.0);
        let h: f32 = tan(theta/2.0);

        let position: Vec3 = vec3(0.0,0.0,0.0);
        let aperture: f32 = 0.6;

        let vwHeight: f32 = 2.0 * h;
        let vwWidth: f32 = aspectRatio * vwHeight;

        let viewDirection: Vec3 = normalize( position - vec3(1.0,0.0,0.0));
        let sideVector: Vec3 = normalize(cross(vec3(0.0,0.0,0.1), viewDirection));
        let upVector: Vec3 = cross(viewDirection, sideVector);

        let horizontal: Vec3 = sideVector * vwWidth * 1.0;
        let vertical: Vec3 = upVector * vwHeight * 1.0;

        return Camera {
            horizontal,
            vertical,
            lowerLeftCorner: position - horizontal/2.0 - vertical/2.0 - viewDirection * viewDirection,
            viewDirection,
            upVector,
            sideVector,
            eyePosition: position,
            lensRadius: aperture / 2.0,
        };
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        return Camera {
            eyePosition: self.eyePosition.clone(),
            viewDirection: self.viewDirection.clone(),
            upVector: self.upVector.clone(),
            sideVector: self.sideVector.clone(),
            lensRadius: self.lensRadius.clone(),
            vertical: self.vertical.clone(),
            horizontal: self.horizontal.clone(),
            lowerLeftCorner: self.lowerLeftCorner.clone(),
        }
    }
}
