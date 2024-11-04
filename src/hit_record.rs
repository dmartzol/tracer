use crate::material::Material;
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
