extern crate rand;

use geometry::{Ray, Vec3};
use hitable::HitRecord;
use rand::Rng;

pub trait Material : Copy + Clone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian{ albedo: albedo }
    }
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal{ albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.point, target - rec.point);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflect = |v: Vec3, n: Vec3| v - n * v.dot(n) * 2.0;
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(1.0, 1.0, 1.0);
    while point.dot(point) >= 1.0 {
        point = Vec3::new(
            rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>())
            * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    point
}
