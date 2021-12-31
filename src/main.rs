mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod tracer;
mod vector;

use camera::Camera;
use hitable::{Hitable, HitableList};
use indicatif::{ProgressBar, ProgressStyle};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use tracer::{clamp, random_float};
use vector::Vector;

fn color(r: &Ray, scene: &HitableList, depth: i64) -> Vector {
    if depth <= 0 {
        return Vector::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = scene.hit(r, 0.001, f64::MAX) {
        if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
            return attenuation.hadamard_product(color(&scattered, scene, depth - 1));
        } else {
            return Vector::new(0.0, 0.0, 0.0);
        }
    } else {
        let t = 0.5 * (r.direction.unit().y + 1.0);
        return (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0);
    }
}

fn write_color(mut color: Vector, samples_per_pixel: i64) {
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    color = color.scale(scale);

    // Gamma-correct for gamma=2.0.
    let r = (256.0 * clamp(color.x.sqrt(), 0.0, 0.999)) as i64;
    let g = (256.0 * clamp(color.y.sqrt(), 0.0, 0.999)) as i64;
    let b = (256.0 * clamp(color.z.sqrt(), 0.0, 0.999)) as i64;

    print!("{} {} {}\n", r, g, b);
}

fn my_scene() -> HitableList {
    let green = Vector::new(0.8, 0.8, 0.0);
    let red = Vector::new(0.7, 0.3, 0.3);
    let grey = Vector::new(0.8, 0.8, 0.8);
    let gold = Vector::new(0.8, 0.6, 0.2);
    let blue = Vector::new(0.1, 0.2, 0.5);

    let material_green = Lambertian::new(green);
    let _material_red = Lambertian::new(red);
    let material_blue = Lambertian::new(blue);
    let _material_silver = Metal::new(grey, 0.3);
    let material_gold = Metal::new(gold, 1.0);
    let material_glass = Dielectric::new(1.5);

    let mut scene = HitableList::default();
    scene.push(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        material_green,
    ));
    scene.push(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5, material_blue));
    scene.push(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.5,
        material_glass,
    ));
    scene.push(Sphere::new(Vector::new(1.0, 0.0, -1.0), 0.5, material_gold));

    return scene;
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new();

    let scene = my_scene();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let progress_bar = ProgressBar::new(image_height as u64);
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("#>-");
    progress_bar.set_style(style);

    for j in (0..image_height).rev() {
        progress_bar.inc(1);
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

    progress_bar.finish_with_message("done");
}
