mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3{ x: -2.0, y: -1.0, z: -1.0 };
    let horizontal = Vec3{ x: 4.0, y: 0.0, z: 0.0 };
    let vertical = Vec3{ x: 0.0, y: 2.0, z: 0.0 };
    let origin = Vec3{ x: 0.0, y: 0.0, z: 0.0 };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray { a: origin,
                b: lower_left_corner + u*horizontal + v*vertical };
            let col = color(&r);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

pub fn color(r: &ray::Ray) -> Vec3 {
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } +
        t * Vec3 { x: 0.5, y: 0.7, z: 1.0 }
}
