extern crate rand;

mod camera;
mod geometry;
mod hitable;
mod material;

use camera::Camera;
use hitable::{Hitable, Sphere};
use geometry::{Ray, Vec3};
use material::Material;
use rand::Rng;
use std::f64;

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut hitable_list: Vec<Box<Hitable>> = Vec::new();
    hitable_list.push(Box::new(
            Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Material::Lambertian(Vec3::new(0.8, 0.8, 0.0))
            )));
    hitable_list.push(Box::new(
            Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Material::Lambertian(Vec3::new(0.8, 0.3, 0.3))
            )));
    hitable_list.push(Box::new(
            Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Material::Metal(Vec3::new(0.8, 0.6, 0.2))
            )));
    hitable_list.push(Box::new(
            Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Material::Metal(Vec3::new(0.8, 0.8, 0.8))
            )));

    let cam = Camera::new();

    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / (nx as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (ny as f64);
                let r = cam.get_ray(u, v);

                col += color(&r, &hitable_list, 0);
            }
            let col = col / ns as f64;
            let ir = (255.99 * col.r().sqrt()) as u8;
            let ig = (255.99 * col.g().sqrt()) as u8;
            let ib = (255.99 * col.b().sqrt()) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}




fn color(ray: &Ray, world: &[Box<Hitable>], depth: u8) -> Vec3 {
    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(rec) => {
            if depth < 50 {
                let vals = material::scatter(rec.material, ray, &rec);
                if vals.is_some() {
                    let (attenuation, scattered) = vals.unwrap();
                    attenuation * color(&scattered, world, depth + 1)
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }

        }
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
