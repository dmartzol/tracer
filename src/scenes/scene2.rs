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

    // Iterate over a grid from -11 to 10 in both the x (a) and z (b) directions
    for a in -11..11 {
        for b in -11..11 {
            // Randomly decide the material type for the current sphere
            let choose_material = random_float();

            // Calculate the center position of the sphere with some random offset
            let center0 = Vector::new(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float(),
            );

            if choose_material < 0.8 {
                // 80% chance to create a diffuse (Lambertian) material

                // Generate a random albedo (color) by taking the Hadamard product of two random vectors
                let albedo = Vector::random().hadamard_product(Vector::random());

                // Create a new Lambertian material with the generated albedo
                let sphere_material = Lambertian::new(albedo);

                // Calculate a second center position for a moving sphere with a slight upward motion
                let center1 = center0 + Vector::new(0.0, random_float_between(0.0, 0.5), 0.0);

                // Add a moving sphere to the scene with the defined material and motion
                scene.push(MovingSphere::new(
                    center0, // Initial center position
                    center1, // Final center position
                    0.0,     // Start time for movement
                    1.0,     // End time for movement
                    0.2,     // Radius of the sphere
                    sphere_material,
                ));
            } else if choose_material < 0.95 {
                // 15% chance to create a metal material

                // Generate a random albedo with each component between 0.5 and 1.0 for metallic colors
                let albedo = Vector::random_between(0.5, 1.0);

                // Generate a random fuzz factor between 0.0 and 0.5 to determine the roughness of the metal
                let fuzz = random_float_between(0.0, 0.5);

                // Create a new Metal material with the generated albedo and fuzz
                let sphere_material = Metal::new(albedo, fuzz);

                // Add a static sphere to the scene with the defined metal material
                scene.push(Sphere::new(
                    center0,         // Center position
                    0.2,             // Radius of the sphere
                    sphere_material, // Material of the sphere
                ));
            } else {
                // 5% chance to create a dielectric (glass) material

                // Create a new Dielectric material with a refractive index of 1.5 (typical for glass)
                let sphere_material = Dielectric::new(1.5);

                // Add a static sphere to the scene with the defined dielectric material
                scene.push(Sphere::new(
                    center0,         // Center position
                    0.2,             // Radius of the sphere
                    sphere_material, // Material of the sphere
                ));
            }
        }
    }

    // add 3 bigger spheres
    let material1 = Dielectric::new(1.5);
    scene.push(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Vector::new(0.4, 0.2, 0.1));
    scene.push(Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0);
    scene.push(Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0, material3));

    return (scene, camera);
}
