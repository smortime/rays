use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub(crate) struct Camera {
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub(crate) fn new(aspect_ratio: f32, image_width: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Point3::origin(),
            pixel00_loc: Point3::origin(),
            pixel_delta_u: Vec3::origin(),
            pixel_delta_v: Vec3::origin(),
        }
    }

    pub(crate) fn render(&mut self, world: impl Hittable) {
        self.initialize();
        dbg!(&self);

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
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f32)
                    + (self.pixel_delta_v * j as f32);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_color = Camera::ray_color(&r, &world);
                write_color(&mut buffer, pixel_color);
            }
        }
        println!("\rDone!\n");
    }

    fn initialize(&mut self) {
        // Calculate image_height and ensure at least 1
        let mut image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }
        self.image_height = image_height;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / image_height as f32);
        self.center = Point3::origin();

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and veritcal delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(&r, &Interval::new(0.0, f32::INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
