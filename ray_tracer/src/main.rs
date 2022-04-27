fn main() {

    // Image
    let width = 256;
    let height = 256;

    // Render
    println!("P3\n{} {}\n255", width, height);
    
    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
