use crate::ray::Ray;
use crate::vector::Vector;

pub struct HitRecord {
    t: f64,
    p: Vector,
    normal: Vector,
}

impl HitRecord {
    pub fn new(t: f64, p: Vector, normal: Vector) -> HitRecord {
        HitRecord { t, p, normal }
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn p(&self) -> Vector {
        self.p
    }
    pub fn normal(&self) -> Vector {
        self.normal
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
