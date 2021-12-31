use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
    pub material: &'a dyn Material,
}

impl HitRecord<'_> {
    pub fn new(t: f64, p: Vector, normal: Vector, material: &dyn Material) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn push(&mut self, v: impl Hitable + 'static) {
        self.list.push(Box::new(v));
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
