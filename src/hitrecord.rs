use vec3::Vec3;
use ray::Ray;

#[derive(Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{ t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0)}
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
