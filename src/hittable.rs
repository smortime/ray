use std::rc::Rc;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub(crate) struct HitRecord {
    p: Point3,
    normal: Option<Vec3>,
    t: f64,
    front_face: Option<bool>,
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
            && self.normal == other.normal
            && self.t == other.t
            && self.front_face == other.front_face
    }
}

impl HitRecord {
    pub(crate) fn new(p: Point3, normal: Option<Vec3>, t: f64) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: None,
        }
    }

    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // NOTE: the param `outward_normal` is assumed to have unit length.
        self.front_face = Some(r.direction().dot(&outward_normal) < 0.0);
        self.normal = if self.front_face.unwrap_or(false) {
            Some(outward_normal)
        } else {
            Some(-outward_normal)
        };
    }

    pub(crate) fn normal(&self) -> Option<Vec3> {
        self.normal
    }
}

pub(crate) trait HitTable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>);
}

pub(crate) struct HitTableList {
    objects: Vec<Rc<dyn HitTable>>,
}

impl HitTable for HitTableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>) {
        let mut temp_rec: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if let (_, Some(hr)) = object.hit(r, &Interval::new(ray_t.min(), closest_so_far)) {
                hit_anything = true;
                closest_so_far = hr.t;
                temp_rec = Some(hr);
            }
        }
        (hit_anything, temp_rec)
    }
}

impl HitTableList {
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.objects.clear()
    }

    pub(crate) fn add(&mut self, object: Rc<dyn HitTable>) {
        self.objects.push(object)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        hittable::HitTable,
        interval::Interval,
        ray::Ray,
        sphere::Sphere,
        vec3::{Point3, Vec3},
    };

    use super::{HitRecord, HitTableList};

    #[test]
    fn test_hit_record() {
        let mut hr = HitRecord::new(Vec3::new(1.0, 1.0, 1.0), None, 1.5);
        assert_eq!(
            hr,
            HitRecord {
                p: Vec3::new(1.0, 1.0, 1.0),
                normal: None,
                t: 1.5,
                front_face: None,
            }
        );

        let ray = Ray::new(Vec3::origin(), Vec3::new(1.0, 1.0, 1.0));
        hr.set_face_normal(&ray, Vec3::new(-1.0, -1.0, -1.0));
        assert!(hr.front_face.unwrap_or(false));
        assert_eq!(hr.normal().unwrap(), Vec3::new(-1.0, -1.0, -1.0));

        hr.set_face_normal(&ray, Vec3::new(2.0, 2.0, 2.0));
        assert!(!hr.front_face.unwrap_or(false));
        assert_eq!(hr.normal().unwrap(), Vec3::new(-2.0, -2.0, -2.0));
    }

    #[test]
    fn test_hit_table_list() {
        let mut world = HitTableList::new();
        assert_eq!(world.objects.len(), 0);
        world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        assert_eq!(world.objects.len(), 1);

        let (hit, hr) = world.hit(
            &Ray::new(Vec3::origin(), Vec3::new(1.0, 1.0, 10.0)),
            &Interval::new(1.0, 3.0),
        );
        assert!(hit);
        assert!(hr.is_some());

        world.clear();
        assert_eq!(world.objects.len(), 0);
    }
}
