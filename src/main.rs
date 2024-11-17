use std::{fs::File, io::BufWriter, io::Write};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let f = File::create("image.ppm").unwrap();
    let mut buffer = BufWriter::new(f);
    buffer
        .write(format!("P3\n{image_height} {image_width}\n255\n").as_bytes())
        .unwrap();

    for j in 0..image_height {
        println!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            buffer
                .write(format!("{ir} {ig} {ib}\n").as_bytes())
                .unwrap();
        }
    }
    println!("\rDone!\n");
}
