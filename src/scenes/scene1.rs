use crate::camera::Camera;
use crate::hitable::HitableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vector::Vector;

pub fn scene() -> (HitableList, Camera) {
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
        0.0,
        1.0,
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
