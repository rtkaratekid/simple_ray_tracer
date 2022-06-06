use ray_tracer::{
    camera::{random_double, Camera},
    hittable_list::HittableList,
    material::{Lambertian, Material, Metal},
    ray::{HitRecord, Hittable, Ray},
    sphere::Sphere,
    vec3::{Color, Point3D},
};

static INFINITY: f64 = f64::INFINITY;
// static PI: f64 = 3.1415926535897932385;

// fn degrees_to_radians(degrees: f64) -> f64 {
//     return degrees * PI / 180.0;
// }

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        if let Some((attenuation, scattered)) = rec.scatter(&r) {
            return attenuation * ray_color(scattered, world, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = Color::unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + (Color::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let left_material = Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let right_material = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let left_sphere = Sphere {
        center: Point3D::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: left_material,
    };
    world.objects.push(&left_sphere);

    let center_sphere = Sphere {
        center: Point3D::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: center_material,
    };
    world.objects.push(&center_sphere);

    let right_sphere = Sphere {
        center: Point3D::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: right_material,
    };
    world.objects.push(&right_sphere);

    let ground = Sphere {
        center: Point3D::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: ground_material,
    };
    world.objects.push(&ground);

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut c = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (width - 1) as f64;
                let v = (j as f64 + random_double()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                c += ray_color(r, &world, max_depth);
            }

            c.write_color(samples_per_pixel);
        }
    }
}
