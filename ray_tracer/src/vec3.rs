use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::camera::{clamp, random_double, range_random_double};

// Type aliases for Vec3
pub type Point3D = Vec3D; // 3D point
pub type Color = Vec3D; // RGB color

#[derive(Debug, Clone, Copy)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { x, y, z }
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random_unit_vector() -> Vec3D {
        random_in_unit_sphere().unit_vector()
    }

    pub fn write_color(self, samples_per_pixel: i32) {
        // Divide the color by the number of samples.
        let scale = 1.0 / samples_per_pixel as f64;

        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        println!(
            "{} {} {}",
            (255.999 * clamp(r, 0.0, 0.999)) as i32,
            (255.999 * clamp(g, 0.0, 0.999)) as i32,
            (255.999 * clamp(b, 0.0, 0.999)) as i32
        )
    }

    pub fn random() -> Vec3D {
        Vec3D::new(random_double(), random_double(), random_double())
    }

    pub fn bounded_random(min: f64, max: f64) -> Vec3D {
        Vec3D::new(
            range_random_double(min, max),
            range_random_double(min, max),
            range_random_double(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn random_in_hemisphere(&self) -> Vec3D {
        let in_unit_sphere = random_in_unit_sphere();
        if in_unit_sphere.dot(*self) > 0.0 {
            // In the same hemisphere as the normal
            return in_unit_sphere;
        }
        -in_unit_sphere
    }
}

pub fn reflect(v: &Vec3D, n: &Vec3D) -> Vec3D {
    *v - (*n * 2 as f64 * v.dot(*n))
}

pub fn random_in_unit_sphere() -> Vec3D {
    loop {
        let p = Vec3D::bounded_random(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

impl Add for Vec3D {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3D {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3D {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3D> for f64 {
    type Output = Vec3D;
    fn mul(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl MulAssign<f64> for Vec3D {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Neg for Vec3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f64> for Vec3D {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div for Vec3D {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<f64> for Vec3D {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
