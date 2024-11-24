use core::f64;
use rand::prelude::*;

pub(crate) fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub(crate) fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub(crate) fn degrees_to_radians(degrees: f64) -> f64 {
    degrees / f64::consts::PI / 180.0
}
