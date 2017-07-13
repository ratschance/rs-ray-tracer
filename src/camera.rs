//! A camera that "views" the scene.
use geometry::{Ray, Vec3};

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
}

impl Camera {
    /// Returns a new `Camera` with default values.
    pub fn new() -> Self {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
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
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
