use crate::tracer::{random_float, random_float_between};
use std::ops;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn add(self, other: Vector) -> Vector {
        return Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }

    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, v: Vector) -> Vector {
        Vector::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn squared_length(self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn magnitude(self) -> f64 {
        self.length()
    }

    pub fn unit(self) -> Vector {
        self / self.length()
    }

    pub fn scale(self, s: f64) -> Vector {
        Vector::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn reverse(self) -> Vector {
        self.scale(-1.0)
    }

    pub fn hadamard_product(self, v: Vector) -> Vector {
        Vector::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

    pub fn random() -> Vector {
        Vector::new(random_float(), random_float(), random_float())
    }

    pub fn random_between(min: f64, max: f64) -> Vector {
        Vector::new(
            random_float_between(min, max),
            random_float_between(min, max),
            random_float_between(min, max),
        )
    }

    pub fn is_near_zero(self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }

    pub fn reflect(self, normal: Vector) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, n: Vector, ni_over_nt: f64) -> Option<Vector> {
        let uv = self.unit();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
        if discriminant > 0.0 {
            let refracted = ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n;
            Some(refracted)
        } else {
            None
        }
    }
}

pub fn random_unit_vector() -> Vector {
    random_in_unit_sphere().unit()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * 3.1415 / 180.0
}

pub fn random_in_unit_sphere() -> Vector {
    loop {
        let p = Vector::random_between(-1.0, 1.0);
        if p.squared_length() >= 1.0 {
            continue;
        }
        return p;
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, v: Vector) -> Vector {
        return Vector::new(self.x + v.x, self.y + v.y, self.z + v.z);
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, v: Vector) -> Vector {
        return Vector::new(self.x - v.x, self.y - v.y, self.z - v.z);
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector {
        return Vector::new(v.x * self, v.y * self, v.z * self);
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;
    fn div(self, f: f64) -> Vector {
        Vector::new(self.x / f, self.y / f, self.z / f)
    }
}
