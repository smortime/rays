use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub(crate) struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub(crate) fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut cloest_so_far = ray_t.max;
        let mut rec = HitRecord::new();

        for object in self.objects.clone() {
            if let Some(r) = object.hit(r, &Interval::new(ray_t.min, cloest_so_far)) {
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
