extern crate nalgebra as na;

mod RrtVec3;
mod RrtColor;

fn main() {

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        
        eprintln!("Scanlines remaining : {}", j);

        for i in 0..image_width {
            let pixel_color : RrtVec3::Vec3 = RrtVec3::Vec3::new([i as f64/ (image_width-1) as f64, j as f64 / (image_height-1) as f64, 0.25]);
            RrtColor::write_color(pixel_color);
        }
    }

    eprintln!("Done");
}