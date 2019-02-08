#![allow(dead_code)]

use crate::core::*;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<Material>) -> Sphere {
        Sphere{center,radius, material}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = dot(&r.direction, &r.direction);
        let b = dot(&oc, &r.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let mut t = (-b - discriminant.sqrt()) / a;

            if t < tmax && t > tmin {
                let p = r.point_at_distance(t);

                return Some(HitRecord {
                t,
                p,
                normal: (p - self.center) / self.radius,
                material: &*self.material,
                });
            }

            t = (-b + discriminant.sqrt()) / a;
            if t < tmax && t > tmin {
                let p = r.point_at_distance(t);

                return Some(HitRecord {
                t,
                p,
                normal: (p - self.center) / self.radius,
                material: &*self.material,
                });
            }
        }

        None
    }
}