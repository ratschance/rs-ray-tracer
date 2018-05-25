extern crate rand;

use geometry::{Ray, Vec3};
use hitable::HitRecord;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64)
}

pub fn scatter(material: Material, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    let reflect = |v: Vec3, n: Vec3| v - (n * v.dot(&n) * 2.0);
    match material {
        Material::Dielectric(ref_idx) => {
            let mut outward_normal: Vec3;
            let reflected = reflect(r_in.direction(), rec.normal);
            let mut ni_over_nt: f64;
            let attenuation = Vec3::one();
            let mut cosine: f64;
            if r_in.direction().dot(&rec.normal) > 0.0 {
                outward_normal = -rec.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * r_in.direction().dot(&rec.normal) / r_in.direction().length();
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / ref_idx;
                cosine = -r_in.direction().dot(&rec.normal) / r_in.direction().length();
            }
            match refract(r_in.direction(), outward_normal, ni_over_nt) {
                Some(refracted) => {
                    let reflect_prob = schlick(cosine, ref_idx);
                    if rand::random::<f64>() < reflect_prob {
                        Some((attenuation, Ray::new(rec.point, reflected)))
                    } else {
                        Some((attenuation, Ray::new(rec.point, refracted)))
                    }
                },
                None => Some((attenuation, Ray::new(rec.point, reflected)))
            }
        }
        Material::Lambertian(attenuation) => {
            let target = rec.point + rec.normal + random_in_unit_sphere();
            let scattered = Ray::new(rec.point, target - rec.point);
            Some((attenuation, scattered))
        }
        Material::Metal(attenuation, fuzz) => {
            let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
            let scattered = Ray::new(rec.point, reflected + random_in_unit_sphere() * fuzz);
            if scattered.direction().dot(&rec.normal) > 0.0 {
                Some((attenuation, scattered))
            } else {
                None
            }
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(&n);
    let descriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if descriminant > 0.0 {
        Some((uv - (n * dt)) * ni_over_nt - n * descriminant.sqrt())
    } else {
        None
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(1.0, 1.0, 1.0);
    while point.dot(&point) >= 1.0 {
        point = Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        ) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    point
}
