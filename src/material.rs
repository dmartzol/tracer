use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vector::{random_in_unit_sphere, random_unit_vector, Vector};
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(v: Vector) -> Lambertian {
        Lambertian { albedo: v }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(v: Vector, f: f64) -> Metal {
        Metal { albedo: v, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector)> {
        let reflected = ray.direction.unit().reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f64, // index of refraction
}

impl Dielectric {
    pub fn new(x: f64) -> Dielectric {
        Dielectric { ir: x }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector)> {
        let attenuation = Vector::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(hit.normal) > 0.0 {
            let cosine = self.ir * ray.direction.dot(hit.normal) / ray.direction.magnitude();
            (hit.normal.reverse(), self.ir, cosine)
        } else {
            let cosine = -ray.direction.dot(hit.normal) / ray.direction.magnitude();
            (hit.normal, 1.0 / self.ir, cosine)
        };

        if let Some(refracted) = ray.direction.refract(outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ir);
            if rand::thread_rng().gen::<f64>() >= reflect_prob {
                let scattered = Ray::new(hit.p, refracted);
                return Some((scattered, attenuation));
            }
        }
        // let reflected = reflect(&ray.direction(), &hit.normal);
        let reflected = ray.direction.reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        Some((scattered, attenuation))
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
