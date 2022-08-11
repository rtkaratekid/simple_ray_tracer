use rand::{thread_rng, Rng};

use crate::{
    ray::Ray,
    vec3::{Point3D, Vec3D},
};

static PI: f64 = 3.1415926535897932385;
fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    let ret: f64 = thread_rng().gen_range(0.0..1.0);
    ret
}

pub fn range_random_double(min: f64, max: f64) -> f64 {
    let ret: f64 = thread_rng().gen_range(min..max);
    ret
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub struct Camera {
    origin: Point3D,
    horizontal: Vec3D,
    vertical: Vec3D,
    lower_left_corner: Point3D,
}

impl Camera {
    pub fn new(
        lookfrom: Point3D,
        lookat: Point3D,
        vup: Vec3D,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // let focal_length = 1.0;

        // let origin = Point3D::new(0.0, 0.0, 0.0);
        let origin = lookfrom;
        // let horizontal = Vec3D::new(viewport_width, 0.0, 0.0);
        let horizontal = viewport_width * u;
        // let vertical = Vec3D::new(0.0, viewport_height, 0.0);
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;
        // let lower_left_corner =
        //     origin - horizontal / 2.0 - vertical / 2.0 - Vec3D::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
