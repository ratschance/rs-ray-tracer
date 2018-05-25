use geometry::{Ray, Vec3};
use material::Material;


pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub parameter: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl HitRecord {
    pub fn new(parameter: f64, point: Vec3, normal: Vec3, material: Material) -> HitRecord {
        HitRecord {
            parameter: parameter,
            point: point,
            normal: normal,
            material: material,
        }
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let descriminant = b * b - a * c;
        if descriminant > 0.0 {
            let sqrt_descriminant = descriminant.sqrt();
            let root = (-b - sqrt_descriminant) / a;
            if root < t_max && root > t_min {
                let parameter = root;
                let point = r.point_at_parameter(parameter);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(parameter, point, normal, self.material));
            }
            let root = (-b + sqrt_descriminant) / a;
            if root < t_max && root > t_min {
                let parameter = root;
                let point = r.point_at_parameter(parameter);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(parameter, point, normal, self.material));
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
                    closest_so_far = t_rec.parameter;
                    hit = Some(t_rec);
                },
                None => {}
            }
        }
        hit
    }
}
