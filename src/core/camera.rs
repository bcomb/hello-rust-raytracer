#![allow(dead_code)]

use crate::core::*;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f32,
    u : Vec3,
    v : Vec3,
    w : Vec3,
}

impl Camera {
    // vfov is top to bottom in degrees
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta/2.0);
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = normalize(lookfrom - lookat);
        let u = normalize(cross(&vup, &w));
        let v = cross(&w, &u);
        let lower_left_corner = origin  - focus_dist*half_width*u - half_height*focus_dist*v - focus_dist*w;
        let horizontal = 2.0*half_width*focus_dist*u;
        let vertical = 2.0*half_height*focus_dist*v;

        Camera { origin, lower_left_corner, horizontal, vertical, lens_radius, u, v, w }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        //Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)

        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd[0] * self.u + rd[1] * self.v;
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)        
    }
}