use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hitable::Hitable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere<M: Material> {
    center: Vector,
    radius: f64,
    material: M,
    bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector, radius: f64, material: M) -> Self {
        let rvec = Vector::new(radius, radius, radius);
        let bbox = Aabb::new_from_points(center - rvec, center + rvec);
        Sphere {
            center,
            radius,
            material,
            bbox,
        }
    }
}

impl<M: Material + Sync> Hitable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let normal = (ray.at(t) - self.center) / self.radius;
                return Some(HitRecord::new(t, ray.at(t), normal, &self.material));
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal, &self.material));
            }
        }
        None
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct MovingSphere<M: Material> {
    center0: Vector,
    center1: Vector,
    time0: f64,
    time1: f64,
    radius: f64,
    material: M,
    bbox: Aabb,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Vector,
        center1: Vector,
        t0: f64,
        t1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        let rvec = Vector::new(radius, radius, radius);
        let box0 = Aabb::new_from_points(center0 - rvec, center0 + rvec);
        let box1 = Aabb::new_from_points(center1 - rvec, center1 + rvec);
        let bbox = Aabb::new_from_bounding_boxes(box0, box1);

        MovingSphere {
            center0,
            center1,
            time0: t0,
            time1: t1,
            radius,
            material,
            bbox,
        }
    }
    pub fn center(&self, t: f64) -> Vector {
        return self.center0
            + ((t - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0);
    }
}

impl<M: Material + Sync> Hitable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t1 && t > t0 {
                let normal = (ray.at(t) - self.center(ray.time())) / self.radius;
                return Some(HitRecord::new(t, ray.at(t), normal, &self.material));
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t1 && t > t0 {
                let p = ray.at(t);
                let normal = (p - self.center(ray.time())) / self.radius;
                return Some(HitRecord::new(t, p, normal, &self.material));
            }
        }
        None
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
