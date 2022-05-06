use crate::vec3::{Point3D, Vec3D};

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

pub struct HitRecord {
    pub p: Point3D,
    pub normal: Vec3D,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3D) {
       self.front_face = r.direction.dot(*outward_normal) < 0.0; 
       self.normal = if self.front_face {*outward_normal} else {-*outward_normal};

    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}