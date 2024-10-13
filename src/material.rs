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

// Lambertian is a diffuse material
impl Lambertian {
    pub fn new(v: Vector) -> Lambertian {
        Lambertian { albedo: v }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction, ray.time());
        Some((scattered, self.albedo))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vector,
    fuzziness_factor: f64,
}

// Metal is a reflective material
impl Metal {
    pub fn new(v: Vector, f: f64) -> Metal {
        Metal {
            albedo: v,
            fuzziness_factor: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector)> {
        let reflected = ray.direction().unit().reflect(hit.normal);
        let scattered = Ray::new(
            hit.p,
            reflected + self.fuzziness_factor * random_in_unit_sphere(),
            ray.time(),
        );
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    refraction_index: f64, // index of refraction
}

// Dielectric is a transparent material
impl Dielectric {
    pub fn new(x: f64) -> Dielectric {
        Dielectric {
            refraction_index: x,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector)> {
        // initialized to (1.0, 1.0, 1.0), representing no loss of light intensity (perfect transparency).
        let attenuation = Vector::new(1.0, 1.0, 1.0);

        // determining ray's interaction direction: checks if the ray is inside the material or outside hitting the surface.
        let ray_is_inside = ray.direction().dot(hit.normal) > 0.0;
        let (outward_normal, ni_over_nt, cosine) = if ray_is_inside {
            // normal is pointing inward, thus ray is inside
            // computes the cosine of the angle between the ray and the normal, adjusted for the refraction index
            let cosine = self.refraction_index * ray.direction().dot(hit.normal)
                / ray.direction().magnitude();
            //  reverses the normal because the ray is inside, and the normal is pointing inward.
            (hit.normal.reverse(), self.refraction_index, cosine)
        } else {
            // normal is pointing outward, thus ray is outside
            let cosine = -ray.direction().dot(hit.normal) / ray.direction().magnitude();
            (hit.normal, 1.0 / self.refraction_index, cosine)
        };

        // refracted ray calculation: attempts to compute the refracted ray using Snell's Law.
        // total internal reflection check: if refract returns None, total internal reflection occurs, and the ray cannot refract.
        if let Some(refracted) = ray.direction().refract(outward_normal, ni_over_nt) {
            // uses Schlick's approximation to estimate the probability that the ray reflects rather than refracts.
            let reflection_probability = schlick(cosine, self.refraction_index);
            // generates a random number to decide whether to reflect or refract based on reflection_probability.
            if rand::thread_rng().gen::<f64>() >= reflection_probability {
                // ff the random number indicates refraction, creates a new refracted Ray.
                let scattered = Ray::new(hit.p, refracted, ray.time());
                // returns the refracted ray and the attenuation vector.
                return Some((scattered, attenuation));
            }
        }

        // if the ray reflects (either by decision or due to total internal reflection), calculates the reflected ray.
        let reflected = ray.direction().reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected, ray.time());
        Some((scattered, attenuation))
    }
}

//  Schlick's approximation for reflectance.
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
