mod hitable;
mod ray;
mod sphere;
mod vector;

use hitable::{Hitable, HitableList};
use ray::Ray;
use sphere::Sphere;
use vector::Vector;

fn color(r: &Ray, world: &HitableList) -> Vector {
    if let Some(hit) = world.hit(r, 0.0, f64::MAX) {
        return 0.5 * hit.normal() + Vector::new(1.0, 1.0, 1.0);
    } else {
        let t = 0.5 * (r.direction().unit().y() + 1.0);
        return (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let image_width: u16 = 200;
    let image_height: u16 = 100;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let lower_left_corner = Vector::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 2.0, 0.0);
    let origin = Vector::new(0.0, 0.0, 0.0);

    let world: HitableList = HitableList::new(vec![
        Box::new(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f64) / (image_width as f64);
            let v = (j as f64) / (image_height as f64);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r, &world);
            let ir = (255.99 * col.x()) as i64;
            let ig = (255.99 * col.y()) as i64;
            let ib = (255.99 * col.z()) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
