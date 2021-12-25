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

pub fn random_float_between(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}
