use crate::{
    hittable::{HitRecord, HitTable},
    interval::Interval,
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug)]
pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
    r_squared: f64,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center
            && self.radius == other.radius
            && self.r_squared == other.r_squared
    }
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius,
            r_squared: radius.powi(2),
        }
    }
}

impl HitTable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>) {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.r_squared;
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return (false, None);
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if ray_t.surrounds(root) {
                return (false, None);
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut hr = HitRecord::new(p, None, root);
        hr.set_face_normal(r, outward_normal);
        (true, Some(hr))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hittable::HitTable,
        interval::Interval,
        ray::Ray,
        vec3::{Point3, Vec3},
    };

    use super::Sphere;

    #[test]
    fn test_sphere_functionality() {
        let s = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        assert_eq!(
            s,
            Sphere {
                center: Point3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                r_squared: 0.25,
            }
        );

        // hit
        let (hit, hr) = s.hit(
            &Ray::new(Vec3::origin(), Vec3::new(1.0, 1.0, 10.0)),
            &Interval::new(1.0, 3.0),
        );
        assert!(hit);
        assert!(hr.is_some());

        // miss
        let (miss, hm) = s.hit(
            &Ray::new(Vec3::origin(), Vec3::new(-1.0, -1.0, -1.0)),
            &Interval::new(1.0, 3.0),
        );
        assert!(!miss);
        assert!(hm.is_none());
    }
}
