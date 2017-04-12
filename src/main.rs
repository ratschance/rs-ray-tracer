mod vec3;
mod ray;
mod hitrecord;
mod sphere;

use vec3::Vec3;
use ray::Ray;
use std::f64;
use hitrecord::Hitable;
use sphere::Sphere;

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut hitable_list: Vec<Box<Hitable>> = Vec::new();
    hitable_list.push(Box::new(Sphere{center: Vec3::new(0.0, -0.0, -1.0), radius: 0.5}));
    hitable_list.push(Box::new(Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray::new(origin, lower_left_corner + (horizontal * u) + (vertical * v));

            let col = color(&r, &hitable_list[..]);
            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &ray::Ray, world: &[Box<Hitable>]) -> Vec3 {
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(rec) => (rec.normal + 1.0) * 0.5,
        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
