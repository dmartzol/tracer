use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hitable: Sync + Clone {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
    bbox: Aabb,
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

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
