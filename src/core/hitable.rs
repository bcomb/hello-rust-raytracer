#![allow(dead_code)]
use crate::core::*;

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    //pub material: Material
}

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}
