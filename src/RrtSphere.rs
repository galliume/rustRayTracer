use std::rc::Rc;

use crate::RrtVec3;
use crate::RrtVec3::Vec3;
use crate::RrtRay::Ray;
use crate::RrtHittable::Hittable;
use crate::RrtHittable::hit_record;

pub fn hit_sphere(center : Vec3, radius : f64, ray : Ray) -> f64 {
    
    let oc : Vec3 = ray.origin() - center;
    let a : f64 = ray.direction().length_squared();
    let half_b : f64 = RrtVec3::dot(oc, ray.direction());
    let c : f64 = oc.length_squared() - radius*radius;
    let discriminant : f64 = half_b * half_b - a * c;

    if 0.0 > discriminant {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center : Vec3,
    radius : f64
}

impl Sphere {
    pub fn new(center : Vec3, radius : f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray : Ray, t_min : f64, t_max : f64, mut rec : hit_record) -> bool {
    
        let oc : Vec3 = ray.origin() - self.center;
        let a : f64 = ray.direction().length_squared();
        let half_b : f64 = RrtVec3::dot(oc, ray.direction());
        let c : f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant : f64 = half_b * half_b - a * c;
    
        if 0.0 > discriminant {
            return false;
        }

        let sqrtd : f64 = discriminant.sqrt();

        let mut root : f64 = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.point_at(rec.t);
        let outward_normal : Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}