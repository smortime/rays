use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utility::{degrees_to_radians, random_f64},
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub(crate) struct Camera {
    aspect_ratio: f64,
    image_width: i64,
    image_height: i64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i64,
    pixel_samples_scale: f64,
    max_depth: i64,
    vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defcous_disk_u: Vec3,
    defcous_disk_v: Vec3,
}

impl Camera {
    pub(crate) fn new(
        aspect_ratio: f64,
        image_width: i64,
        samples_per_pixel: i64,
        max_depth: i64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            center: Point3::origin(),
            pixel00_loc: Point3::origin(),
            pixel_delta_u: Vec3::origin(),
            pixel_delta_v: Vec3::origin(),
            pixel_samples_scale: 0.0,
            vfov,
            lookfrom,
            lookat,
            vup,
            u: Vec3::origin(),
            v: Vec3::origin(),
            w: Vec3::origin(),
            defocus_angle,
            focus_dist,
            defcous_disk_u: Vec3::origin(),
            defcous_disk_v: Vec3::origin(),
        }
    }

    pub(crate) fn render(&mut self, world: impl Hittable) {
        self.initialize();

        let f = File::create("image.ppm").unwrap();
        let mut buffer = BufWriter::new(f);
        buffer
            .write_all(
                format!("P3\n{0} {1}\n255\n", self.image_width, self.image_height).as_bytes(),
            )
            .unwrap();

        for j in 0..self.image_height {
            println!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::origin();
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, &world, self.max_depth);
                }
                write_color(&mut buffer, pixel_color * self.pixel_samples_scale);
            }
        }
        println!("\rDone!\n");
    }

    fn initialize(&mut self) {
        // Calculate image_height and ensure at least 1
        let mut image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
        if image_height < 1 {
            image_height = 1;
        }
        self.image_height = image_height;

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Camera
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        // Calculate u, v, w unit basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * self.v;

        // Calculate the horizontal and veritcal delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defcous_disk_u = self.u * defocus_radius;
        self.defcous_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i64, j: i64) -> Ray {
        // Construct a camera ray originating from the defocus disk directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defcous_disk_u) + (p.y() * self.defcous_disk_v)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, world: &dyn Hittable, depth: i64) -> Color {
        if depth < 0 {
            return Color::origin();
        }
        if let Some(rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            if let Some(m) = &rec.mat {
                if let Some((s, a)) = m.scatter(r, &rec) {
                    return a * Self::ray_color(&s, world, depth - 1);
                }
            }
            let direction = rec.normal + Vec3::random_unit_vector();
            return 0.6 * Self::ray_color(&Ray::new(rec.p, direction), world, depth - 1);
        }
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
