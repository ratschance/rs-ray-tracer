extern crate rand;

use geometry::{Ray, Vec3};
use hitable::HitRecord;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3),
}

pub fn scatter(material: Material, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    match material {
        Material::Lambertian(attenuation) => {
            let target = rec.point + rec.normal + random_in_unit_sphere();
            let scattered = Ray::new(rec.point, target - rec.point);
            Some((attenuation, scattered))
        }
        Material::Metal(attenuation) => {
            let reflect = |v: Vec3, n: Vec3| v - n * v.dot(n) * 2.0;
            let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
            let scattered = Ray::new(rec.point, reflected);
            if scattered.direction().dot(rec.normal) > 0.0 {
                Some((attenuation, scattered))
            } else {
                None
            }
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(1.0, 1.0, 1.0);
    while point.dot(point) >= 1.0 {
        point = Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        ) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    point
}
