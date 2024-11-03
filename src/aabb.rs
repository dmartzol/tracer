use crate::interval::Interval;
use crate::ray::Ray;
// use crate::tracer::order_pair;
use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new() -> Aabb {
        Aabb::default()
    }
    pub fn new_from_intervals(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }
    pub fn new_from_points(a: Vector, b: Vector) -> Aabb {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        Aabb {
            x: Interval::new(a.x(), b.x()),
            y: Interval::new(a.y(), b.y()),
            z: Interval::new(a.z(), b.z()),
        }
    }

    pub fn axis_interval(&self, n: i64) -> Interval {
        if n == 0 {
            return self.x;
        } else if n == 1 {
            return self.y;
        }
        return self.z;
    }

    pub fn hit(self, r: Ray, ray_t: Interval) -> bool {
        for axis in 0..2 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.direction().axis_value(axis);

            let t0 = (ax.start() - r.origin().axis_value(axis)) * adinv;
            let t1 = (ax.end() - r.origin().axis_value(axis)) * adinv;

            let mut start = ray_t.start();
            let mut end = ray_t.end();

            if t0 < t1 {
                if t0 > ray_t.start() {
                    start = t0;
                }
                if t1 < ray_t.end() {
                    end = t1;
                }
            } else {
                if t1 > ray_t.start() {
                    start = t1;
                }
                if t0 < ray_t.end() {
                    end = t0;
                }
            }
            if end <= start {
                return false;
            }
        }
        return true;
    }

    // pub fn hit(self, r: Ray, ray: Interval) -> bool {
    //     // Iterate over all three axes (x, y, z)
    //     for axis in 0..3 {
    //         // Get the interval bounds for the current axis (either x, y, or z)
    //         let ax = self.axis_interval(axis);
    //         // Get the direction value for the current axis
    //         let adinv = r.direction().axis_value(axis);

    //         // Check if the direction is zero to avoid division by zero
    //         if adinv == 0.0 {
    //             // If the ray direction is zero, it must lie within the axis bounds to intersect
    //             if r.origin().axis_value(axis) < ax.start()
    //                 || r.origin().axis_value(axis) > ax.end()
    //             {
    //                 return false; // The ray is parallel to this axis and outside the interval
    //             }
    //             continue; // Move on to the next axis as there is no intersection in this one
    //         }

    //         // Calculate the reciprocal of the direction value for efficiency
    //         let adinv = 1.0 / adinv;

    //         // Calculate intersection distances (t0 and t1) along the current axis
    //         let t0 = (ax.start() - r.origin().axis_value(axis)) * adinv;
    //         let t1 = (ax.end() - r.origin().axis_value(axis)) * adinv;

    //         // Ensure t0 is the smaller value and t1 is the larger value using a utility function
    //         let (t0, t1) = order_pair(t0, t1);

    //         // Update the start and end of the valid intersection interval
    //         let start = ray.start().max(t0);
    //         let end = ray.end().min(t1);

    //         // If the interval is invalid (end is less than start), return false
    //         if end < start {
    //             return false;
    //         }
    //     }
    //     // If all axis checks are passed, the ray intersects the bounding box
    //     true
    // }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import functions from the parent module

    #[test]
    fn test_hit() {
        #[derive(Debug)]
        struct TestCase {
            #[allow(dead_code)]
            desc: &'static str,
            input_interval: Interval,
            input_ray: Ray,
            should_hit: bool,
        }

        let cases = vec![TestCase {
            desc: "first case",
            input_interval: Interval::new(0.0, 1.0),
            input_ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0), 0.0),
            should_hit: false,
        }];

        for case in cases {
            let aabb = Aabb::new();
            let is_hit = aabb.hit(case.input_ray, case.input_interval);
            if case.should_hit && !is_hit {}
            assert_eq!(is_hit, case.should_hit, "failed on case: {:?}", case);
        }
    }
}
