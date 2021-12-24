use std::ops;

#[derive(Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn add(self, other: Vector) -> Vector {
        return Vector {
            x: self.x() + other.x(),
            y: self.y() + other.y(),
            z: self.z() + other.z(),
        };
    }

    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, v: Vector) -> Vector {
        Vector::new(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        )
    }

    pub fn squared_length(self) -> f64 {
        self.x().powf(2.0) + self.y().powf(2.0) + self.z().powf(2.0)
    }

    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit(self) -> Vector {
        self / self.length()
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
