use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};

pub(crate) struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub(crate) fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub(crate) fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.objects.clear();
    }

    pub(crate) fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut cloest_so_far = ray_tmax;
        let mut rec = HitRecord::new();

        for object in self.objects.clone() {
            if let Some(r) = object.hit(r, ray_tmin, cloest_so_far) {
                hit_anything = true;
                cloest_so_far = r.t;
                rec = r;
            }
        }
        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
}
