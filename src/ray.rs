use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn direction(self) -> Vector {
        self.direction
    }

    pub fn origin(self) -> Vector {
        self.origin
    }

    pub fn at(self, t: f64) -> Vector {
        self.origin() + t * self.direction()
    }
}
