#![allow(dead_code)]

use crate::core::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere{center,radius}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        None
    }
}