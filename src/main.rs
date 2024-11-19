mod color;
mod ray;
mod vec3;

use crate::vec3::Vec3;
use color::write_color;
use ray::Ray;
use vec3::Point3;

use crate::color::Color;
use std::{fs::File, io::BufWriter, io::Write};

fn ray_color(r: Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate image_height and ensure at least 1
    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::origin();

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and veritcal delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let f = File::create("image.ppm").unwrap();
    let mut buffer = BufWriter::new(f);
    buffer
        .write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())
        .unwrap();

    for j in 0..image_height {
        println!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f32) + (pixel_delta_v * j as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(r);
            write_color(&mut buffer, pixel_color);
        }
    }
    println!("\rDone!\n");
}
