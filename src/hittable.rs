use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub(crate) struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn new() -> Self {
        Self {
            p: Point3::origin(),
            normal: Vec3::origin(),
            t: 0.0,
            front_face: false,
        }
    }

    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
