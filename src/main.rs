mod color;
mod vec3;

use color::write_color;

use crate::color::Color;
use std::{fs::File, io::BufWriter, io::Write};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let f = File::create("image.ppm").unwrap();
    let mut buffer = BufWriter::new(f);
    buffer
        .write_all(format!("P3\n{image_height} {image_width}\n255\n").as_bytes())
        .unwrap();

    for j in 0..image_height {
        println!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.0,
            );
            write_color(&mut buffer, pixel_color);
        }
    }
    println!("\rDone!\n");
}
