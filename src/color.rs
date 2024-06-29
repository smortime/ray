use std::fs::File;
use std::io::Write;

use crate::vec3::Vec3;

pub(crate) type Color = Vec3;

const NUMBER: f64 = 255.999;

impl Color {
    pub(crate) fn write_color(&self, f: &mut File) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let ir = (NUMBER * r) as i16;
        let ig = (NUMBER * g) as i16;
        let ib = (NUMBER * b) as i16;

        writeln!(f, "{} {} {}", ir, ig, ib).unwrap();
    }
}
