use crate::RrtVec3::Vec3;

pub fn write_color(v: Vec3) {
    println!("{} {} {}", (255.999 * v.x()) as i64, (255.999 * v.y()) as i64, (255.999 * v.z()) as i64);
}