use crate::vector::Vector;
use rand::Rng; // 0.8.0

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn random_float() -> f64 {
    return random_float_between(0.0, 1.0);
}

pub fn random_integer(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn random_float_between(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

pub fn write_color(mut color: Vector, samples_per_pixel: i64) {
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    color = color.scale(scale);

    // Gamma-correct for gamma=2.0.
    let r = (256.0 * clamp(color.x().sqrt(), 0.0, 0.999)) as i64;
    let g = (256.0 * clamp(color.y().sqrt(), 0.0, 0.999)) as i64;
    let b = (256.0 * clamp(color.z().sqrt(), 0.0, 0.999)) as i64;

    print!("{} {} {}\n", r, g, b);
}

// Utility function to ensure t0 is always the smaller value and t1 is the larger value
pub fn order_pair(a: f64, b: f64) -> (f64, f64) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
