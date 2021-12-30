use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vector::{random_unit_vector, Vector};

pub trait Material {
    fn scatter(self, hit: &HitRecord) -> Option<(Ray, Vector)>;
}

pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(v: Vector) -> Lambertian {
        Lambertian { albedo: v }
    }
}

impl Material for Lambertian {
    fn scatter(self, hit: &HitRecord) -> Option<(Ray, Vector)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}