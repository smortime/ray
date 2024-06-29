use std::{f64::INFINITY, fs::File, io::Write};

use crate::{
    color::Color,
    hittable::HitTable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub(crate) struct Camera {
    image_width: i16,
    image_height: i16,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub(crate) fn new(image_width: i16, aspect_ratio: f64) -> Self {
        let temp = (image_width as f64 / aspect_ratio) as i16;
        let image_height = if temp >= 1 { temp } else { 1 };
        let center = Point3::origin();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub(crate) fn render(&self, world: &dyn HitTable) {
        let mut file = match File::create("image.ppm") {
            Err(e) => panic!("couldn't create file: {}", e),
            Ok(f) => f,
        };

        writeln!(&mut file, "P3").unwrap();
        writeln!(&mut file, "{} {}", self.image_width, self.image_height).unwrap();
        writeln!(&mut file, "255").unwrap();

        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_color = ray_color(&r, world);
                pixel_color.write_color(&mut file);
            }
        }
    }
}

fn ray_color(r: &Ray, world: &dyn HitTable) -> Color {
    if let (true, Some(rec)) = world.hit(r, &Interval::new(0.0, INFINITY)) {
        return 0.5 * (rec.normal().unwrap() + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::Camera;

    #[test]
    fn test_creating_camera() {
        let camera = Camera::new(400, 16.0 / 9.0);
        debug_assert_eq!(camera.image_width, 400);
        debug_assert_eq!(camera.image_height, 225);
        debug_assert_eq!(camera.center.clone(), Vec3::origin());

        let pixel00_loc = camera.pixel00_loc;
        assert_eq!(pixel00_loc.x().round(), -2.0);
        assert_eq!(pixel00_loc.y().round(), 1.0);
        assert_eq!(pixel00_loc.z().round(), -1.0);
    }
}
