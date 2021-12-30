use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere<M: Material> {
    center: Vector,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
    pub fn normal_at(&self, p: Vector) -> Vector {
        (p - self.center).unit()
    }
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - a * c;
        if discriminant > 0.0 {
            let root = (-b - discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                let p = r.at(root);
                let outward_normal = self.normal_at(p);
                let record = HitRecord::new(root, p, outward_normal, false, &self.material);
                record.set_face_normal(r, outward_normal);
                return Some(record);
            }
            let root = (-b + discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                let p = r.at(root);
                let outward_normal = self.normal_at(p);
                let record = HitRecord::new(root, p, outward_normal, false, &self.material);
                record.set_face_normal(r, outward_normal);
                return Some(record);
            }
        }
        return None;
    }
}
