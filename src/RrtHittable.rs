use crate::RrtRay::Ray;
use crate::RrtVec3;
use crate::RrtVec3::Vec3;

#[derive(Copy, Clone)]
pub struct hit_record {
    pub p : Vec3,
    pub normal : Vec3,
    pub t : f64,
    pub front_face : bool,
}

impl hit_record {
    pub fn new() -> hit_record {
        hit_record{
            p : Vec3::new([0.0, 0.0, 0.0]),
            normal : Vec3::new([0.0, 0.0, 0.0]),
            t : 0.0,
            front_face : false
        }
    }
    pub fn set_face_normal(mut self, ray : Ray, outward_normal : Vec3) {
        self.front_face = RrtVec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray : Ray, t_min : f64, t_max : f64, rec : hit_record) -> bool;
}