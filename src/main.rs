mod camera;
mod hitable;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use hitable::{Hitable, HitableList};
use rand::Rng; // 0.8.0
use ray::Ray;
use sphere::Sphere;
use vector::Vector;

fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0);
}

fn color(r: &Ray, world: &HitableList) -> Vector {
    if let Some(hit) = world.hit(r, 0.0, f64::MAX) {
        return 0.5 * (hit.normal() + Vector::new(1.0, 1.0, 1.0));
    } else {
        let t = 0.5 * (r.direction().unit().y() + 1.0);
        return (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;

    // Camera
    let camera = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    // Scene
    let world: HitableList = HitableList::new(vec![
        Box::new(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vector::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_float()) / (image_width - 1) as f64;
                let v = (j as f64 + random_float()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + color(&ray, &world);
            }
            let scale = 1.0 / samples_per_pixel as f64;

            let ir = (255.99 * pixel_color.x() * scale) as i64;
            let ig = (255.99 * pixel_color.y() * scale) as i64;
            let ib = (255.99 * pixel_color.z() * scale) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
