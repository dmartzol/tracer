use crate::hitable::{Hitable, HitableList};
use crate::vector::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Vector {
        self.origin + t * self.direction
    }

    pub fn color(&self, scene: &HitableList, depth: i64) -> Vector {
        if depth <= 0 {
            return Vector::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = scene.hit(&self, 0.001, f64::MAX) {
            if let Some((scattered, attenuation)) = hit.material.scatter(&self, &hit) {
                return attenuation.hadamard_product(scattered.color(scene, depth - 1));
            } else {
                return Vector::new(0.0, 0.0, 0.0);
            }
        } else {
            let t = 0.5 * (self.direction.unit().y() + 1.0);
            return (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0);
        }
    }
}
