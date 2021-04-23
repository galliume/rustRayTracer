mod RrtVec3;
mod RrtColor;
mod RrtRay;

use crate::RrtVec3::Vec3;
use crate::RrtRay::Ray;

fn main() {

    // Image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let image_width : i64 = 400;
    let image_height : i64 = image_width / aspect_ratio as i64;

    // Camera
    let viewport_height : f64 = 2.0;
    let  viewport_width : f64 = aspect_ratio * viewport_height as f64;
    let  focal_length : f64 = 1.0;

    let  origin : Vec3 = Vec3::new([0.0, 0.0, 0.0]);
    let  horizontal : Vec3 = Vec3::new([viewport_width, 0.0, 0.0]);
    let  vertical : Vec3 = Vec3::new([0.0, viewport_height, 0.0]);
    let  lower_left_corner : Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new([0.0, 0.0, focal_length]);

    // Render

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        
        eprintln!("Scanlines remaining : {}", j);

        for i in 0..image_width {
            let u : f64 = i as f64 / (image_width - 1) as f64;
            let v : f64 = j as f64 / (image_height - 1) as f64;

            let direction : Vec3 = lower_left_corner + Vec3::new([u, u, u]) * horizontal + Vec3::new([v, v, v]) * vertical - origin;
            let ray : Ray = Ray::new(origin, direction);

            let pixel_color : Vec3 = RrtRay::ray_color(ray);
            RrtColor::write_color(pixel_color);
        }
    }

    eprintln!("Done");
}