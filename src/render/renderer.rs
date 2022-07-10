use std::thread;
use glm::{IVec2, IVec4, ivec4, normalize, Vec3};
use rand::{Rng, thread_rng};
use sfml::graphics::Image;
use crate::Camera;
use crate::math::ray::Ray;
use crate::math::vColor::VColor;
use crate::model::hitRecord::HitRecord;
use crate::render::scene::Scene;

pub struct Renderer {
    sampling: i32,
    rayRecursionDepth: i16,

    scene: Scene,
    pub imageBuffer: Image,
    viewPort: IVec4,
}

impl Renderer {
    pub fn new(sc: Scene, res: &IVec2, img: Image) -> Renderer {
        let renderer: Renderer = Renderer {
            scene: sc,
            viewPort: ivec4(0, 0, res.x.clone(), res.y.clone()),
            imageBuffer: img,
            ..Default::default()
        };

        return renderer;
    }

    pub fn colorRay(&self, x: i32, y: i32, ray: &Ray, depth: i16) -> VColor {
        if depth as i32 <= 0 { return VColor::new_sc(0.0); };

        let mut hitRecord: HitRecord = HitRecord::default();

        if self.scene.Hit(ray, &mut hitRecord) {
            let mut scattered: Ray = Ray::default();
            let mut attenuation: VColor = VColor::default();
            if hitRecord.material.scatter(ray, &hitRecord, &mut attenuation, &mut scattered) {
                return attenuation * self.colorRay(x, y, &scattered, depth - 1);
            }
            return VColor::default();
        }

        let unitDirection: Vec3 = normalize(ray.direction);
        let t: f32 = 0.5 * (unitDirection.y + 1.0);
        return VColor::new_sc(1.0) * (1.0 - t) + VColor::new_rgb(0.5, 0.7, 1.0) * t;
    }

    pub fn render(&mut self) {
        for y in 0..self.viewPort.w {
            for x in 0..self.viewPort.z {
                let mut num = VColor::default();


                for s in 0..self.sampling {
                    //let ray: Ray = self.scene.getCamera().createARay(x as f32 + thread_rng().gen_range(0.0..1.0),y as f32 + thread_rng().gen_range(0.0..1.0));
                    let ray: Ray = self.scene.getCamera().createARay((x as f32 + thread_rng().gen_range(0.0..1.0) as f32)  / (self.viewPort.z - 1) as f32
                                                                     , (y as f32 + thread_rng().gen_range(0.0..1.0) as f32) / (self.viewPort.w - 1) as f32);

                    if x == 0 && y == 0 { println!("LowerLeft: {}, {}, {}", ray.direction.x, ray.direction.y, ray.direction.z) };
                    if x == 799 && y == 0 { println!("LowerRight: {}, {}, {}", ray.direction.x, ray.direction.y, ray.direction.z) };
                    if x == 0 && y == 599 { println!("UpperLeft: {}, {}, {}", ray.direction.x, ray.direction.y, ray.direction.z) };
                    if x == 799 && y == 599 { println!("UpperRight: {}, {}, {}", ray.direction.x, ray.direction.y, ray.direction.z) };

                    num += self.colorRay(x, y, &ray, self.rayRecursionDepth);
                }

                let result: VColor = num / self.sampling;
                self.imageBuffer.set_pixel(x as u32, y as u32, &result.asSFColor());
            }
        };
    }
}

impl Default for Renderer {
    fn default() -> Self {
        return Self {
            scene: Scene::new(Camera::default(), vec![]),
            sampling: 16,
            imageBuffer: Image::new(800, 600).unwrap(),
            rayRecursionDepth: 8,
            viewPort: ivec4(0, 0, 800, 600),
        };
    }
}