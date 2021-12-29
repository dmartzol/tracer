use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl HitRecord<'_> {
    pub fn new(
        t: f64,
        p: Vector,
        normal: Vector,
        front_face: bool,
        material: &dyn Material,
    ) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            front_face,
            material,
        }
    }
    pub fn set_face_normal(mut self, r: &Ray, outward_normal: Vector) {
        if r.direction().dot(outward_normal) > 0.0 {
            // ray is inside the object
            self.normal = -1.0 * outward_normal;
            self.front_face = false;
        } else {
            // ray is outside the object
            self.normal = outward_normal;
            self.front_face = true;
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { list }
    }
    pub fn push(mut self, v: Box<dyn Hitable>) {
        self.list.push(v);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        return hit_anything;
    }
}
