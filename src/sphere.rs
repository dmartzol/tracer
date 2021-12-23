use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
    pub fn center(self) -> Vector {
        self.center
    }
    pub fn radius(self) -> f64 {
        self.radius
    }
    pub fn normal_at(self, p: Vector) -> Vector {
        (p - self.center).unit()
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
            let root = (-b - discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                let p = r.at(root);
                let outward_normal = self.normal_at(p);
                let record = HitRecord::new(root, p, outward_normal, false);
                record.set_face_normal(r, outward_normal);
                return Some(record);
            }
            let root = (-b + discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                let p = r.at(root);
                let outward_normal = self.normal_at(p);
                let record = HitRecord::new(root, p, outward_normal, false);
                record.set_face_normal(r, outward_normal);
                return Some(record);
            }
        }
        return None;
    }
}
