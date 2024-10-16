use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use tracer::scenes;
use tracer::tracer::{random_float, write_color};
use tracer::vector::Vector;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let (scene, camera) = scenes::scene1::scene();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let progress_bar = ProgressBar::new((image_height * image_width) as u64);
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:100.cyan/blue} {percent}% {pos:>7}/{len:7} [{eta_precise}] {msg}",
    )
    .unwrap()
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
