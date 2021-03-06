use std::borrow::Borrow;
use std::rc::Rc;
use std::thread;
use std::thread::{JoinHandle, Thread};
use glm::{IVec2, length, vec3, Vec3};
use sfml::graphics::{Color, Image, RenderTarget, RenderWindow, Sprite, Text, Texture};
use sfml::window::event::Event;
use sfml::window::{ContextSettings, VideoMode, WindowStyle};
use crate::math::vColor::VColor;
use crate::model::dielectric::Dielectric;
use crate::model::hittableObject::HittableObject;
use crate::model::lambertian::Lambertian;
use crate::model::metal::Metal;
use crate::model::sphere::Sphere;
use crate::render::camera::Camera;
use crate::render::renderer::Renderer;
use crate::render::scene::Scene;

mod render;
mod model;
mod math;


fn main() {
    let resolution: IVec2 = IVec2 { x: 800, y: 600 };

    let mut window: RenderWindow = RenderWindow::new(VideoMode {
        width: resolution.x as u32,
        height: resolution.y as u32,
        bits_per_pixel: 8,
    }, "SFML WORKS!", WindowStyle::default(), &ContextSettings::default()).unwrap();


    let img: Image = Image::new(resolution.x as u32, resolution.y as u32).unwrap();

    let camPos: Vec3 = vec3(13.0,-2.0,-2.0);
    let lookAt: Vec3 = vec3(0.0,0.0,0.0);

    let cam: Camera = Camera::new(camPos,lookAt,resolution.x, resolution.y, 0.1, 10.0, 20.0);

    let mut scene: Scene = Scene::randomScene();
    scene.setCam(cam);

    let mut renderer: Renderer = Renderer::new(scene, &resolution, img);

    renderer.render();

    loop {
        if window.is_open() {
            let event: Event = window.poll_event();

            if event == Event::Closed {
                //TODO worker.join().unwrap();
                window.close();
                break;
            }


            let optTexture: Option<Texture> = Texture::new_from_image(&renderer.imageBuffer);
            let mut tex: Texture;

            if optTexture.is_some() {
                tex = optTexture.unwrap();
            } else {
                println!("Texture is invalid!");
                window.close();
                break;
            }

            let sprite: Option<Sprite> = Sprite::new_with_texture(&tex);

            window.draw(sprite.expect("Sprite is invalid!").borrow());
            window.display();
        }
    }
}
