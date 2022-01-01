use crate::ray::Ray;
use crate::vector::{degrees_to_radians, random_in_unit_disk, Vector};

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vector,
        lookat: Vector,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin: lookfrom,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
