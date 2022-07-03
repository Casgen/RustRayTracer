use std::ops::Deref;
use glm::{Mat4, mat4};
use crate::math::ray::Ray;
use crate::model::hitRecord::HitRecord;
use crate::model::hittableObject::HittableObject;
use crate::render::camera::Camera;

pub struct Scene {
    cam: Camera,
    modelMat: Mat4,
    hittableObjects: Vec<Box<dyn HittableObject>>,
    
    tMax: f32,
    tMin: f32
}

impl Scene {
        
    pub fn new(camera: Camera, objects: Vec<Box<dyn HittableObject>>) -> Self {
        return Self {
            cam: camera,
            hittableObjects: objects,
            ..Default::default()
        }
    }
    
    pub fn new_cam(camera: Camera) -> Self {
        return Self {
            cam: camera,
            ..Default::default()
        }
    }
    
    pub fn Hit(&self, ray: &Ray, hitRecord: &mut HitRecord) -> (bool) {
        let mut hitAnything: bool = false;
        let mut closestSoFar: f32 = self.tMax;
        
        for object in self.hittableObjects.iter() {
            if object.testIntersection(ray,self.tMin,closestSoFar,hitRecord) {
                hitAnything = true;
                closestSoFar = hitRecord.t;
            }
        }
        
        return hitAnything;
    }
    
    pub fn getCamera(&self) -> &Camera {
        return &self.cam;
    }
}

impl Default for Scene {
    fn default() -> Self {
       return Self {
           cam: Camera::default(),
           modelMat: mat4(
               1.0, 0.0, 0.0, 0.0,
               0.0, 1.0, 0.0, 0.0,
               0.0, 0.0, 1.0, 0.0,
               0.0, 0.0, 0.0, 1.0,
           ),
           hittableObjects: vec![],
           tMax: f32::MAX,
           tMin: 0.001
       } 
    }
}
/*
impl Clone for Scene {
    fn clone(&self) -> Self {
        
        let mut clonedObjects: Vec<Box<dyn HittableObject>> = Vec::with_capacity(self.hittableObjects.capacity());
        
        clonedObjects.clone_from_slice(&self.hittableObjects[0..1]);
        
       return Self {
           cam: self.cam.clone(),
           modelMat: self.modelMat.clone(),
           hittableObjects: clonedObjects,
           tMax: 0.0,
           tMin: 0.0
       } 
    }
}*/