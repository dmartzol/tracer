mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod tracer;
mod vector;

use camera::Camera;
use hitable::{Hitable, HitableList};
use material::Lambertian;
use ray::Ray;
use sphere::Sphere;
use std::time::Instant;
use tracer::{clamp, random_float};
use vector::{random_unit_vector, Vector};

fn color(r: &Ray, scene: &HitableList, depth: i64) -> Vector {
    if depth <= 0 {
        return Vector::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = scene.hit(r, 0.001, f64::MAX) {
        let target = hit.normal + hit.p + random_unit_vector();
        return 0.5 * color(&Ray::new(hit.p, target - hit.p), scene, depth - 1);
    } else {
        let t = 0.5 * (r.direction().unit().y() + 1.0);
        return (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0);
    }
}

fn write_color(mut color: Vector, samples_per_pixel: i64) {
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    color = color.scale(scale);

    // Gamma-correct for gamma=2.0.
    let r = (256.0 * clamp(color.x().sqrt(), 0.0, 0.999)) as i64;
    let g = (256.0 * clamp(color.y().sqrt(), 0.0, 0.999)) as i64;
    let b = (256.0 * clamp(color.z().sqrt(), 0.0, 0.999)) as i64;

    print!("{} {} {}\n", r, g, b);
}

fn main() {
    let now = Instant::now();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let camera = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let scene: HitableList = HitableList::new(vec![
        Box::new(Sphere::new(
            Vector::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Vector::new(0.8, 0.3, 0.3)),
        )),
        Box::new(Sphere::new(
            Vector::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Vector::new(0.8, 0.8, 0.0)),
        )),
    ]);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vector::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_float()) / (image_width - 1) as f64;
                let v = (j as f64 + random_float()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + color(&ray, &scene, max_depth);
            }

            write_color(pixel_color, samples_per_pixel);
        }
    }

    let _elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
