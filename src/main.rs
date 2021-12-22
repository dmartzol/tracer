mod hitable;
mod ray;
mod vector;

use hitable::{HitRecord, Hitable, HitableList, Sphere};
use ray::Ray;
use vector::Vector;

fn hit_sphere(center: Vector, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius.powf(2.0);
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn color(r: &Ray, world: &Hitable) -> Vector {
    let rec: HitRecord;
    if world.hit(r, 0.0, 1000.0, &rec) {
        return 0.5
            * Vector::new(
                rec.normal().x() + 1.0,
                rec.normal().y() + 1.0,
                rec.normal().z() + 1.0,
            );
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

    let sphere1 = Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0);
    let world: HitableList;
    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f64) / (image_width as f64);
            let v = (j as f64) / (image_height as f64);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            // let col = color(&r, world);
            // let ir = (255.99 * col.x()) as i64;
            // let ig = (255.99 * col.y()) as i64;
            // let ib = (255.99 * col.z()) as i64;
            // print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
