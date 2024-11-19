use crate::vec3::Vec3;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub(crate) type Color = Vec3;

pub(crate) fn write_color(buff: &mut BufWriter<File>, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (r * 255.999) as i32;
    let gbyte = (g * 255.999) as i32;
    let bbyte = (b * 255.999) as i32;

    buff.write_all(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())
        .unwrap();
}
