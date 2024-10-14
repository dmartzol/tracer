use crate::camera::Camera;
use crate::hitable::HitableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::{MovingSphere, Sphere};
use crate::tracer::{random_float, random_float_between};
use crate::vector::Vector;

pub fn scene() -> (HitableList, Camera) {
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
        0.0,
        1.0,
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
                let center2 = center + Vector::new(0.0, random_float_between(0.0, 0.5), 0.0);
                scene.push(MovingSphere::new(
                    center,
                    center2,
                    0.0,
                    1.0,
                    0.2,
                    sphere_material,
                ));
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
