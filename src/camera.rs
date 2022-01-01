use crate::ray::Ray;
use crate::vector::{degrees_to_radians, Vector};

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(
        lookfrom: Vector,
        lookat: Vector,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Camera {
            origin: lookfrom,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lookfrom - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }
    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
