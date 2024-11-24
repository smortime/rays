use crate::interval::Interval;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub(crate) type Color = Vec3;

pub(crate) fn write_color(buff: &mut BufWriter<File>, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0, 255]
    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i64;
    let gbyte = (256.0 * intensity.clamp(g)) as i64;
    let bbyte = (256.0 * intensity.clamp(b)) as i64;

    buff.write_all(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())
        .unwrap();
}
