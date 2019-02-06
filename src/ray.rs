use vec3;

pub struct Ray {
    origin: Vec3d,
    direction: Vec3
}

impl Ray {
    pub fn point_at_distance(&self, d: f32) -> Vec3 {
        origin + d * direction
    }
}
