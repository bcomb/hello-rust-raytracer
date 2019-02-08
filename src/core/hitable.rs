#![allow(dead_code)]
use crate::core::*;

#[derive(Debug)]
pub struct HitRecord<'a> {
    pub t: f32,                     // distance
    pub p: Vec3,                    // intersection point
    pub normal: Vec3,
    pub material: &'a Material      // material of the hit surface
}

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}


pub fn closest_hit<'a>(list: &'a Vec<Box<Hitable>>,r: &Ray, tmin: f32, tmax: f32 ) -> Option<HitRecord<'a> > {
    let mut closest_hit : Option<HitRecord<'a>> = None;
    let closest_distance = std::f32::MAX;
    for hitable in list
    {
        if let Some(hit_record) = hitable.hit(r,tmin,tmax) {
            if hit_record.t < closest_distance {
                closest_hit = Some(hit_record);
            }
        }
    }

    closest_hit
}