extern crate rand;

pub mod camera;
pub mod geometry;
mod hitable;
mod material;

use camera::Camera;
use geometry::{Ray, Vec3};
use hitable::{Hitable, Sphere};
use material::Material;
use rand::Rng;
use std::f64;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
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
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(width) / f64::from(height),
        aperture,
        dist_to_focus,
    );
    let mut rng = rand::thread_rng();
    let mut pixel_data: Vec<(u8, u8, u8)> = Vec::new();

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(width);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(height);
                let r = cam.get_ray(u, v);

                col += color(&r, &hitable_list, 0);
            }
            let col = col / f64::from(ns);
            let ir = (255.99 * col.r().sqrt()) as u8;
            let ig = (255.99 * col.g().sqrt()) as u8;
            let ib = (255.99 * col.b().sqrt()) as u8;
            pixel_data.push((ir, ig, ib));
        }
    }
    let output_path = Path::new("output.ppm");
    match File::create(&output_path) {
        Ok(file) => write_ppm(file, width, height, pixel_data),
        Err(e) => println!("Could not create file - {}", e.to_string()),
    }
}

fn gen_random_scene() -> Vec<Box<dyn Hitable>> {
    let mut hitable_list: Vec<Box<dyn Hitable>> = Vec::new();
    hitable_list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Vec3::new(0.5, 0.5, 0.5)),
    )));
    // Randomly generate others
    let reference_vec = Vec3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(
                f64::from(a) + 0.9 * rand::random::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rand::random::<f64>(),
            );
            if (center - reference_vec).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let lam = Vec3::new(
                        rand::random::<f64>() * rand::random::<f64>(),
                        rand::random::<f64>() * rand::random::<f64>(),
                        rand::random::<f64>() * rand::random::<f64>(),
                    );
                    hitable_list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(lam),
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let metal = Vec3::new(
                        0.5 * (1.0 + rand::random::<f64>()),
                        0.5 * (1.0 + rand::random::<f64>()),
                        0.5 * (1.0 + rand::random::<f64>()),
                    );
                    hitable_list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(metal, 0.5 * rand::random::<f64>()),
                    )));
                } else {
                    // Glass
                    hitable_list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(1.5),
                    )));
                }
            }
        }
    }
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

fn color(ray: &Ray, world: &[Box<dyn Hitable>], depth: u8) -> Vec3 {
    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(rec) => {
            if depth < 50 {
                if let Some((attenuation, scattered)) = material::scatter(rec.material, ray, &rec) {
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
