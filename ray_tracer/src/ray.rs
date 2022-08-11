use crate::{
    material::{Lambertian, Material, Scatterable},
    vec3::{Color, Point3D, Vec3D},
};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3D) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3D {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3D,
    pub normal: Vec3D,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3D::new(0.0, 0.0, 0.0),
            normal: Vec3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            front_face: false,
            material: Material::Lambertian(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3D) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn scatter(&self, r_in: &Ray) -> Option<(Color, Ray)> {
        self.material.scatter(r_in, self)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
