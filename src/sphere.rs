use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
    pub fn center(&self) -> Vector {
        self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center();
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center()) / self.radius();
                return Some(HitRecord::new(t, p, normal));
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center()) / self.radius();
                return Some(HitRecord::new(t, p, normal));
            }
        }
        return None;
    }
}
