use vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: vec3::Vec3,
    pub direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Self {
        Ray { origin: origin, direction: direction }
    }

    pub fn origin(&self) -> vec3::Vec3 {
        self.origin
    }

    pub fn direction(&self) -> vec3::Vec3 {
        self.direction
    }

    pub fn point_at_parameter(&self, distance: f64) -> vec3::Vec3 {
        self.origin + self.direction * distance
    }
}
