use std::f32::consts::PI;
use glm::{cos, cross, IVec4, ivec4, mat4, Mat4, normalize, radians, sin, vec3, Vec3, Vec4, vec4};
use glm::ext::{look_at, perspective};
use crate::math::ray::Ray;

pub struct Camera {
    eyePosition: Vec3,
    viewDirection: Vec3,
    upVector: Vec3,
    sideVector: Vec3,

    viewPort: IVec4,
    viewMat: Mat4,
    modelMat: Mat4,
    projMatPersp: Mat4,

    azimuth: f32,
    zenith: f32,

}


impl Camera {
    pub fn new(width: i32, height: i32) -> Camera {
        let mut cam: Camera = Camera {
            viewPort: ivec4(0, 0, width, height),
            projMatPersp: perspective::<f32>(radians(45.0), width as f32 / height as f32, 0.1, 100.0),
            ..Default::default()
        };
        
        cam.calculateAndSetUpVectors();
        cam.calculateAndSetViewTransform();
        return cam;
    }

    fn calculateAndSetUpVectors(&mut self) {
        self.viewDirection = vec3(cos(self.azimuth) * cos(self.zenith),
                                  sin(self.azimuth) * cos(self.zenith),
                                  sin(self.zenith));

        self.upVector = vec3(cos(self.azimuth) * cos(self.zenith + PI / 2.0),
                             sin(self.azimuth) * cos(self.zenith + PI / 2.0),
                             sin(self.zenith + PI / 2.0));
        self.sideVector = cross(self.viewDirection, self.upVector);
    }

    fn calculateAndSetViewTransform(&mut self) {
        self.viewMat = look_at(self.eyePosition, self.viewDirection, self.upVector);
    }
    
    pub fn unProject(&self, coords: &Vec3) -> Vec3 {
        
        let inverse: Mat4 = glm::inverse(&(self.projMatPersp * self.modelMat * self.viewMat));
        
        let mut tmp: Vec4 = vec4(coords.x,coords.y,coords.z, 1.0);
        
        tmp.x = (tmp.x - self.viewPort[0] as f32) / self.viewPort[2] as f32;
        tmp.y = (tmp.y - self.viewPort[1] as f32) / self.viewPort[3] as f32;
        tmp = tmp * 2.0 - 1.0;
        
        let mut obj: Vec4 = inverse * tmp;
        
        obj = obj / obj.w;
        
        return vec3(obj.x,obj.y,obj.z);
    }

    pub fn createARay(&self, x: f32, y: f32) -> Ray {
        return Ray::new(self.eyePosition, normalize(self.unProject(&vec3(x as f32, y as f32,0.1))));
    }
    
}

impl Default for Camera {
    fn default() -> Self {
        let eyeVec: Vec3 = vec3(0.0, 0.0, 0.0);
        let viewVec: Vec3 = vec3(1.0, 0.0, 0.0);
        let upVec: Vec3 = vec3(0.0, 0.0, 1.0);

        return Camera {
            viewMat: look_at(eyeVec, viewVec, upVec),
            eyePosition: eyeVec,
            viewDirection: viewVec,
            upVector: upVec,
            sideVector: vec3(0.0, -1.0, 0.0),
            viewPort: ivec4(0, 0, 800, 600),
            projMatPersp: perspective::<f32>(radians(45.0), 800.0 / 600.0, 0.1, 100.0),
            modelMat: mat4(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            azimuth: 0.0,
            zenith: 0.0,
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
            viewPort: self.viewPort.clone(),
            viewMat: self.viewMat.clone(),
            modelMat: self.modelMat.clone(),
            projMatPersp: self.projMatPersp.clone(),
            azimuth: self.azimuth,
            zenith: self.zenith
        }
    }
}
