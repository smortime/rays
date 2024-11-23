use core::f32;

#[derive(Debug)]
pub(crate) struct Interval {
    pub(crate) min: f32,
    pub(crate) max: f32,
}

impl Interval {
    pub(crate) fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub(crate) fn empty() -> Self {
        Self::new(f32::INFINITY, -f32::INFINITY)
    }

    pub(crate) fn size(&self) -> f32 {
        self.max - self.min
    }

    pub(crate) fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub(crate) fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub(crate) fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
