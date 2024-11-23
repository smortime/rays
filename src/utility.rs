use core::f32;
use rand::prelude::*;

pub(crate) fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub(crate) fn random_f32_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_f32()
}

pub(crate) fn degrees_to_radians(degrees: f32) -> f32 {
    degrees / f32::consts::PI / 180.0
}
