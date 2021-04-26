use crate::RrtVec3;
use crate::RrtVec3::Vec3;
use crate::RrtRay::Ray;

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