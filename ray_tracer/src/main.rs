use ray_tracer::vec3::{Color, Vec3};
// use ray_tracer::vec3::Vec3;
fn main() {
    // Image
    let width = 256;
    let height = 256;

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel_color = Color::new(
                i as f64 / (width - 1) as f64,
                j as f64 / (height - 1) as f64,
                0.25,
            );

            pixel_color.write_color();
       }
    }

}
