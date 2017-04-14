extern crate rand;

mod geometry;
mod hitable;
mod camera;

use camera::Camera;
use hitable::{Hitable, Sphere};
use geometry::{Ray, Vec3};
use rand::Rng;
use std::f64;

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut hitable_list: Vec<Box<Hitable>> = Vec::new();
    hitable_list.push(Box::new(Sphere{center: Vec3::new(0.0, -0.0, -1.0), radius: 0.5}));
    hitable_list.push(Box::new(Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}));

    let cam = Camera::new();

    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / (nx as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (ny as f64);
                let r = cam.get_ray(u, v);

                col += color(&r, &hitable_list);
            }
            let col = col / ns as f64;
            let ir = (255.99 * col.r().sqrt()) as u8;
            let ig = (255.99 * col.g().sqrt()) as u8;
            let ib = (255.99 * col.b().sqrt()) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    while p.dot(p) >= 1.0 {
        p = Vec3::new(
            rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>())
            * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}


fn color(r: &Ray, world: &[Box<Hitable>]) -> Vec3 {
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            let ray = Ray::new(rec.p, target - rec.p);
            color(&ray, world) * 0.5
        }
        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
