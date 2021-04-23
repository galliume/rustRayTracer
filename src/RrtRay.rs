use crate::RrtVec3::Vec3;
use crate::RrtVec3::unit_vector;

pub fn ray_color(ray: Ray) -> Vec3 {
    let unit_direction : Vec3 = unit_vector(ray.direction());

    let t : f64 = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new([1.0-t, 1.0-t, 1.0-t]) * Vec3::new([1.0, 1.0, 1.0]) + Vec3::new([t, t, t]) * Vec3::new([0.5, 0.7, 1.0]);
}

pub struct Ray {
    orig : Vec3,
    dir: Vec3
}

impl Ray {
    pub fn new(origin : Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.orig + Vec3::new([t, t, t]) * self.dir
    }
}