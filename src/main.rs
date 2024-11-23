mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utility;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::empty();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(16.0 / 9.0, 400, 100);
    cam.render(world);
}
