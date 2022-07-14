use std::ops::Deref;
use std::ptr::null;
use std::rc::Rc;
use glm::{length, Mat4, mat4, Vec3, vec3, vec4};
use rand::{Rng, thread_rng};
use crate::{Dielectric, Lambertian, Metal, Sphere, VColor};
use crate::math::ray::Ray;
use crate::model::hitRecord::HitRecord;
use crate::model::hittableObject::HittableObject;
use crate::model::material::Material;
use crate::render::camera::Camera;

pub struct Scene {
    cam: Camera,
    hittableObjects: Vec<Box<dyn HittableObject>>,
    tMax: f32,
    tMin: f32,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Box<dyn HittableObject>>) -> Self {
        return Self {
            cam: camera,
            hittableObjects: objects,
            ..Default::default()
        };
    }

    pub fn new_cam(camera: Camera) -> Self {
        return Self {
            cam: camera,
            ..Default::default()
        };
    }

    pub fn setCam(&mut self, camera: Camera) { self.cam = camera}

    pub fn Hit(&self, ray: &Ray, hitRecord: &mut HitRecord) -> (bool) {
        let mut hitAnything: bool = false;
        let mut closestSoFar: f32 = self.tMax;

        for object in self.hittableObjects.iter() {
            if object.testIntersection(ray, self.tMin, closestSoFar, hitRecord) {
                hitAnything = true;
                closestSoFar = hitRecord.t;
            }
        }

        return hitAnything;
    }

    pub fn getCamera(&self) -> &Camera {
        return &self.cam;
    }

    pub fn addObject(&mut self, object: Box<dyn HittableObject>) {
        self.hittableObjects.push(object);
    }

    pub fn randomScene() -> Self {
        let mut scene: Self = Self::default();

        let groundMaterial: Lambertian = Lambertian::new(VColor::new_sc(0.5));

        scene.addObject(Box::new(Sphere::new(1000.0, vec3(0.0, 0.0, 1000.0), Rc::new(groundMaterial))));

        for a in -11..11 {
            for b in -11..11 {
                let chooseMat: f64 = thread_rng().gen();
                let center: Vec3 = vec3(a as f32 + 0.9 * thread_rng().gen::<f32>(), b as f32 + 0.9 * thread_rng().gen::<f32>(),-0.2);

                if length(center - vec3(4.0, 0.2, 0.0)) > 0.9 {
                    let sphereMat: Rc<dyn Material>;

                    if chooseMat < 0.8 {
                        let albedo: VColor = VColor::random() * VColor::random();
                        sphereMat = Rc::new(Lambertian::new(albedo));
                    } else if chooseMat < 0.95 {
                        let albedo: VColor = VColor::random_ran(0.5..1.0);
                        let fuzz: f32 = thread_rng().gen_range(0.0..0.5);

                        sphereMat = Rc::new(Metal::new(albedo, fuzz));
                    } else {
                        sphereMat = Rc::new(Dielectric::new(1.5));
                    }

                    scene.addObject(Box::new(Sphere::new(0.2, center, sphereMat)));
                }
            }
        }

        let mat1: Rc<Dielectric> = Rc::new(Dielectric::new(1.5));
        scene.addObject(Box::new(Sphere::new(1.0, vec3(0.0, 0.0, -1.0), mat1)));

        let mat2: Rc<Lambertian> = Rc::new(Lambertian::new(VColor::new_rgb(0.4, 0.2, 0.1)));
        scene.addObject(Box::new(Sphere::new(1.0, vec3(-4.0, 0.0, -1.0), mat2)));

        let mat3: Rc<Metal> = Rc::new(Metal::new(VColor::new_rgb(0.7, 0.6, 0.5), 0.0));
        scene.addObject(Box::new(Sphere::new(1.0, vec3(4.0, 0.0, -1.0), mat3)));


        return scene;
    }
}

impl Default for Scene {
    fn default() -> Self {
        return Self {
            cam: Camera::default(),
            hittableObjects: vec![],
            tMax: f32::MAX,
            tMin: 0.001,
        };
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