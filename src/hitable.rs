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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool;
}

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { list }
    }
    pub fn push(self, v: Box<Hitable>) {
        self.list.push(v);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool {
        let mut closest_so_far = t_max;
        let temp_rec;
        let hit_anything = false;
        for h in self.list.iter() {
            if h.hit(r, t_min, closest_so_far, &temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }
        return hit_anything;
    }
}
