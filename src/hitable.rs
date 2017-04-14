use geometry::{Ray, Vec3};


pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{ t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0)}
    }
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

impl Hitable for [Box<Hitable>] {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit: Option<HitRecord> = None;
        for v in self.iter() {
            match v.hit(r, t_min, closest_so_far) {
                Some(t_rec) => {
                    closest_so_far = t_rec.t;
                    hit = Some(t_rec);
                },
                None => {}
            }
        }
        hit
    }
}
