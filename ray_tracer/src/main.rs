use ray_tracer::{
    ray::Ray,
    vec3::{Color, Point3D, Vec3D},
};

fn hit_sphere(center: Point3D, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;

    let a = r.direction.length_squared();
    let half_b = r.direction.dot(oc);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt() ) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3D::new(0.0,0.0,-1.0), 0.5, &r);
    if t > 0.0 {
        let n = Color::unit_vector(r.at(t) - Vec3D::new(0.0,0.0,-1.0));
        return 0.5*Color::new(n.x+1.0, n.y+1.0, n.z+1.0);
    }
    let unit_direction = Color::unit_vector(r.direction);
    let t = 0.5*(unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + (Color::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3D::new(0.0, 0.0, 0.0);
    let horizontal = Vec3D::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3D::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3D::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            ray_color(ray).write_color();
        }
    }
}
