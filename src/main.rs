extern crate rand;

pub mod camera;
pub mod geometry;
mod hitable;
mod material;

use camera::Camera;
use hitable::{Hitable, Sphere};
use geometry::{Ray, Vec3};
use material::Material;
use rand::Rng;
use std::error::Error;
use std::f64;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let width: u32 = 1200;
    let height: u32 = 800;
    let ns: u32 = 50;

    let hitable_list = gen_random_scene();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom,
                            lookat,
                            Vec3::new(0.0, 1.0, 0.0),
                            20.0,
                            width as f64 / height as f64,
                            aperture,
                            dist_to_focus);
    let mut rng = rand::thread_rng();
    let mut pixel_data: Vec<(u8, u8, u8)> = Vec::new();

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / (width as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (height as f64);
                let r = cam.get_ray(u, v);

                col += color(&r, &hitable_list, 0);
            }
            let col = col / ns as f64;
            let ir = (255.99 * col.r().sqrt()) as u8;
            let ig = (255.99 * col.g().sqrt()) as u8;
            let ib = (255.99 * col.b().sqrt()) as u8;
            pixel_data.push((ir, ig, ib));
        }
    }
    let output_path = Path::new("output.ppm");
    match File::create(&output_path) {
        Ok(file) => write_ppm(file, width, height, pixel_data),
        Err(why) => println!("Could not create file - {}", why.description())
    }
}

fn gen_random_scene() -> Vec<Box<Hitable>> {
    let mut hitable_list: Vec<Box<Hitable>> = Vec::new();
    hitable_list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Vec3::new(0.5, 0.5, 0.5)),
    )));
    // Randomly generate others
    hitable_list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(1.5),
    )));
    hitable_list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Vec3::new(0.4, 0.2, 0.1)),
    )));
    hitable_list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    )));
    hitable_list
}

fn write_ppm(file: File, width: u32, height: u32, data: Vec<(u8, u8, u8)>) {
    let mut writer = BufWriter::new(&file);
    writeln!(writer, "P3\n{} {}\n255", width, height).expect("Failed to write header");
    for (r, g, b) in data {
        writeln!(writer, "{} {} {}", r, g, b).expect("Failed to write line");
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
                    Vec3::zero()
                }
            } else {
                Vec3::zero()
            }
        }
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::one() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
