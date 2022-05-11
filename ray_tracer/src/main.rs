use ray_tracer::{
    camera::{random_double, Camera},
    hittable_list::HittableList,
    ray::{HitRecord, Hittable, Ray},
    sphere::Sphere,
    vec3::{random_in_unit_sphere, Color, Point3D, Vec3D},
};

static INFINITY: f64 = f64::INFINITY;
static PI: f64 = 3.1415926535897932385;

fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

fn hit_sphere(center: Point3D, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;

    let a = r.direction.length_squared();
    let half_b = r.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + random_in_hemisphere(rec.normal);
        // let target = rec.p + rec.normal + random_unit_vector();
        // let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = Color::unit_vector(r.direction);
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

    let mut world = HittableList {
        objects: Vec::new(),
    };

    let sphere1 = Sphere {
        center: Point3D::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    world.objects.push(&sphere1);

    let sphere2 = Sphere {
        center: Point3D::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    world.objects.push(&sphere2);

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut c = Color::new(0.0, 0.0, 0.0);

            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (width - 1) as f64;
                let v = (j as f64 + random_double()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                c += ray_color(r, &world, max_depth);
            }

            c.write_color(samples_per_pixel);
        }
    }
}
