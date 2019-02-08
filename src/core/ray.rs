#![allow(dead_code)]
use crate::core::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    // Constructor ensure normalized direction
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray{ origin: origin, direction: normalize(direction) }
    }

    pub fn point_at_distance(&self, d: f32) -> Vec3 {
        self.origin + self.direction * d
    }
}
