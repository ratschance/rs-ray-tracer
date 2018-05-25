//! Geometry representations.
//!
//! The [`Vec3`] struct represents a vector in 3-space. It
//! includes a set of operations that simplify its usage
//! in the ray tracer along with operator overrides that
//! behave as expected.
//!
//! The [`Ray`] struct represents a ray with a vector
//! defining its origin and a vector defining its direction.
//!
//! [`Vec3`]: struct.Vec3.html
//! [`Ray`]: struct.Ray.html

use std::ops::{Add, AddAssign, Neg, Mul, Div, Sub};

/// A representation of a vector in 3-space.
///
/// This `struct` is instantiated by the [`new`] function. See its
/// documentation for more information.
///
/// [`new`]: struct.Vec3.html#method.new
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A representation of a ray.
///
/// This `struct` is instantiated by the [`new`] function. See its
/// documentation for more information.
///
/// [`new`]: struct.Ray.html#method.new
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new Ray with the given origin and direction.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    /// Returns the origin of the Ray.
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// Returns the direction of the Ray.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Returns the point at the specified parameter.
    ///
    /// Implements the function `p(t) = A + t*B`, where `A`
    /// is the origin, `B` is the direction, and `t` is the
    /// specified parameter. This is used to get the location
    /// of the point as it moves across the ray.
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl Vec3 {
    /// Creates a new Vec3 with the given x, y, and z coords.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn one() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    /// Gets the `r` component of the vector.
    ///
    /// Used when manipulating the vector as a RGB color.
    pub fn r(&self) -> f64 {
        self.x
    }

    /// Gets the `g` component of the vector.
    ///
    /// Used when manipulating the vector as a RGB color.
    pub fn g(&self) -> f64 {
        self.y
    }

    /// Gets the `b` component of the vector.
    ///
    /// Used when manipulating the vector as a RGB color.
    pub fn b(&self) -> f64 {
        self.z
    }

    /// Returns the length of the vector.
    ///
    /// This can also be referred to as the magnitude of the
    /// vector.
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    /// Returns the sum of each component squared.
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the dot product of this and another vector.
    ///
    /// # Parameters
    /// * `other` - Vec3 representing the second vector in the
    /// dot product.
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x
        }
    }

    /// Returns a unit vector.
    ///
    /// Unit vector will have the same direction as the vector
    /// this is called on.
    pub fn unit_vector(&self) -> Self {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Vec3::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        Vec3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}
