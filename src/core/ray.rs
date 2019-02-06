use crate::core::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn point_at_distance(&self, d: f32) -> Vec3 {
        self.origin + self.direction * d
    }
}
