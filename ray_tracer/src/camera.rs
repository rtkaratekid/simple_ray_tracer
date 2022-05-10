use rand::{
    thread_rng,
    Rng,
};

use crate::{vec3::{Point3D, Vec3D}, ray::Ray};

pub fn random_double() -> f64 {
    let ret: f64 = thread_rng().gen_range(0.0..1.0);
    ret
}

pub fn range_random_double(min:f64, max: f64) -> f64 {
    let ret: f64 = thread_rng().gen_range(min..max);
    ret
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}

pub struct Camera {
    origin: Point3D,
    horizontal: Vec3D,
    vertical: Vec3D,
    lower_left_corner: Point3D,
}

impl Camera {

    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3D::new(0.0, 0.0, 0.0);
        let horizontal = Vec3D::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3D::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3D::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
