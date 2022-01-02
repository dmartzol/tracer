mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod tracer;
mod vector;

use camera::Camera;
use hitable::HitableList;
use indicatif::{ProgressBar, ProgressStyle};
use material::{Dielectric, Lambertian, Metal};
use rayon::prelude::*;
use sphere::Sphere;
use tracer::{clamp, random_float, random_float_between};
use vector::Vector;

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

fn my_scene() -> (HitableList, Camera) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let lookfrom = Vector::new(-2.0, 2.0, 1.0);
    let lookat = Vector::new(0.0, 0.0, -1.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let green = Vector::new(0.8, 0.8, 0.0);
    let red = Vector::new(0.7, 0.3, 0.3);
    let grey = Vector::new(0.8, 0.8, 0.8);
    let gold = Vector::new(0.8, 0.6, 0.2);
    let blue = Vector::new(0.1, 0.2, 0.5);
    let white = Vector::new(1.0, 1.0, 1.0);

    let material_green = Lambertian::new(green);
    let _material_red = Lambertian::new(red);
    let _material_blue = Lambertian::new(blue);
    let _material_white = Lambertian::new(white);
    let material_silver = Metal::new(grey, 0.0);
    let material_gold = Metal::new(gold, 0.0);
    let material_glass = Dielectric::new(1.5);

    let mut scene = HitableList::default();
    scene.push(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        material_green,
    ));
    scene.push(Sphere::new(
        Vector::new(0.0, 0.0, -1.0),
        0.5,
        material_silver,
    ));
    scene.push(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.5,
        material_glass,
    ));
    scene.push(Sphere::new(Vector::new(1.0, 0.0, -1.0), 0.5, material_gold));

    return (scene, camera);
}

fn _random_scene() -> (HitableList, Camera) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let lookfrom = Vector::new(13.0, 2.0, 3.0);
    let lookat = Vector::new(0.0, 0.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut scene = HitableList::default();
    let ground_material = Lambertian::new(Vector::new(0.5, 0.5, 0.5));
    scene.push(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Vector::new(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float(),
            );

            if choose_mat < 0.8 {
                // diffuse
                let albedo = Vector::random().hadamard_product(Vector::random());
                let sphere_material = Lambertian::new(albedo);
                scene.push(Sphere::new(center, 0.2, sphere_material));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Vector::random_between(0.5, 1.0);
                let fuzz = random_float_between(0.0, 0.5);
                let sphere_material = Metal::new(albedo, fuzz);
                scene.push(Sphere::new(center, 0.2, sphere_material));
            } else {
                // glass
                let sphere_material = Dielectric::new(1.5);
                scene.push(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    scene.push(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Vector::new(0.4, 0.2, 0.1));
    scene.push(Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0);
    scene.push(Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0, material3));

    return (scene, camera);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let (scene, camera) = my_scene();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let progress_bar = ProgressBar::new((image_height * image_width) as u64);
    let style = ProgressStyle::default_bar()
        .template(
            "[{elapsed_precise}] {bar:100.cyan/blue} {percent}% {pos:>7}/{len:7} [{eta_precise}] {msg}",
        )
        .progress_chars("#>-");
    progress_bar.set_style(style);

    let mut screen = vec![Vector::default(); image_height * image_width];

    screen
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            let i = index % image_width;
            let j = image_height - index / image_width;
            let mut pixel_color = Vector::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_float()) / (image_width - 1) as f64;
                let v = (j as f64 + random_float()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray.color(&scene, max_depth);
            }
            *pixel = pixel_color;
            progress_bar.inc(1);
        });

    for pixel_color in screen {
        write_color(pixel_color, samples_per_pixel);
    }

    progress_bar.finish_with_message("");
}
