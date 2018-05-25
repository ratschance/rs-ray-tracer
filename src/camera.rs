//! A camera that "views" the scene.

extern crate rand;
use geometry::{Ray, Vec3};
use std::f64;

/// The camera struct.
///
/// # Fields
/// * `origin` - The origin of the scene.
/// * `lower_left_corner` - The bottom left corner of the scene.
/// * `horizontal` - The maximum x coordinate of the scene.
/// * `vertical` - The maximum y coordinate of the scene.
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Returns a new `Camera` with default values.
    pub fn new(lookfrom: Vec3, lookat: Vec3, view_up: Vec3, view_fov: f64,
                aspect: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = view_fov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = (view_up.cross(&w)).unit_vector();
        let v = w.cross(&u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical:  v * 2.0 * half_height * focus_dist,
            lens_radius: aperture / 2.0,
        }
    }

    /// Returns a ray from the origin to the computed endpoint.
    ///
    /// # Parameters
    /// * `u` - Horizontal offset from the left boundary of the scene.
    /// * `v` - Vertical offset from the bottom of the scene.
    ///
    /// `u` and `v` are offsets used to compute the endpoint of the
    /// ray starting from the lower left corner.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = random_in_unit_disc() * self.lens_radius;
        let offset = u * rd.x + v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        }
    }
}

fn random_in_unit_disc() -> Vec3{
    let mut p: Vec3;
    loop {
        p = Vec3::new(rand::random::<f64>(), rand::random::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 {
            break;
        }
    }
    p
}
