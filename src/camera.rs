use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;
        let origin = Vector::new(0.0, 0.0, 0.0);
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        Camera {
            origin: origin,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vector::new(0.0, 0.0, focal_length),
            horizontal: horizontal,
            vertical: vertical,
        }
    }
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}