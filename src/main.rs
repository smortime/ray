mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utilities;
mod vec3;

use crate::camera::Camera;
use crate::hittable::HitTableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut world = HitTableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let aspect_ratio = 16.0 / 9.0;
    let cam = Camera::new(400, aspect_ratio);
    cam.render(&world);

    let elapsed = now.elapsed();
    println!("\n\nElapsed: {:.2?}", elapsed);
    println!("\rDone.               \n")
}
