use core::f64;

#[derive(Debug)]
pub(crate) struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    pub(crate) fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub(crate) fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub(crate) fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
