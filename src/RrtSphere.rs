use crate::RrtVec3;
use crate::RrtVec3::Vec3;
use crate::RrtRay::Ray;

pub fn hit_sphere(center : Vec3, radius : f64, ray : Ray) -> f64 {
    
    let oc : Vec3 = ray.origin() - center;
    let a : f64 = RrtVec3::dot(ray.direction(), ray.direction());
    let b : f64 = 2.0 * RrtVec3::dot(oc, ray.direction());
    let c : f64 = RrtVec3::dot(oc, oc) - radius*radius;
    let discriminant : f64 = b * b - 4.0 * a * c;

    if 0.0 > discriminant {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}