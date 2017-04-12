use vec3::Vec3;
use hitrecord::{Hitable, HitRecord};
use ray::Ray;


#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = HitRecord::new();
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius*self.radius;
        let descriminant = b*b - a*c;
        if descriminant > 0.0 {
            let sqrt_descriminant = descriminant.sqrt();
            let temp = (-b - sqrt_descriminant)/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }
            let temp = (-b + sqrt_descriminant)/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }
        }
        None
    }
}
