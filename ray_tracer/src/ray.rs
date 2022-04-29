use crate::vec3::{Point3D, Vec3D};

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
