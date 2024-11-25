use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub(crate) trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

pub(crate) struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub(crate) fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scattered_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scattered_direction);
        Some((scattered, self.albedo))
    }
}

pub(crate) struct Metal {
    albedo: Color,
}

impl Metal {
    pub(crate) fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.direction().reflection(&rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        Some((scattered, self.albedo))
    }
}
