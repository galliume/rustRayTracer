use crate::RrtVec3::Vec3;
use crate::RrtVec3::unit_vector;
use crate::RrtSphere::hit_sphere;

pub fn ray_color(ray: Ray) -> Vec3 {

    let point : Vec3 = Vec3::new([0.0, 0.0, -1.0]);

    let t : f64 = hit_sphere(point, 0.5, ray);

    if 0.0 < t {
        let n : Vec3 = unit_vector(ray.point_at(t) - point);
        return Vec3::new([0.5, 0.5, 0.5]) * Vec3::new([n.x() + 1.0, n.y() + 1.0, n.z() +1.0]);
    }

    let unit_direction : Vec3 = unit_vector(ray.direction());
    let t : Vec3 = Vec3::new([0.5, 0.5, 0.5]) * (unit_direction + Vec3::new([0.0, 1.0, 0.0]));

    return (Vec3::new([1.0, 1.0, 1.0]) - t) * (Vec3::new([1.0, 1.0, 1.0]) + t) * Vec3::new([0.5, 0.7, 1.0]);
}

#[derive(Debug, Copy, Clone)]
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