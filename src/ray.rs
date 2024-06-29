use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub(crate) struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

impl Ray {
    pub(crate) fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn origin(&self) -> Point3 {
        self.origin
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.direction
    }

    pub(crate) fn at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::{Point3, Vec3};

    use super::Ray;

    #[test]
    fn test_ray_functionality() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(
            r,
            Ray {
                origin: Vec3::new(1.0, 2.0, 3.0),
                direction: Vec3::new(1.0, 1.0, 1.0)
            }
        );

        assert_eq!(r.origin(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(r.direction(), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(r.at(1.0), Point3::new(2.0, 3.0, 4.0));
    }
}
