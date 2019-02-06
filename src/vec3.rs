#![allow(dead_code)]

use std::ops;
use std::cmp;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f32; 3]
}

impl Vec3 {
    // Static function
    pub fn new(x: f32, y: f32, z:f32) -> Vec3 {
        Vec3 { e : [x, y, z] }
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn normalize(&mut self) {
        let k = 1.0 / self.squared_length();
        self.e[0] /= k;
        self.e[1] /= k;
        self.e[2] /= k;
    }

    pub fn lerp(v1: &Vec3, v2: &Vec3, a: f32) -> Vec3 {
        Vec3 {
        e:  [
            v1.e[0] + (v2.e[0] - v1.e[0]) * a,
            v1.e[1] + (v2.e[1] - v1.e[1]) * a,
            v1.e[2] + (v2.e[2] - v1.e[2]) * a
            ]
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]] }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]] }
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2]] }
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, rhs: &Vec3) -> bool {
        self.e[0] == rhs.e[0] && self.e[1] == rhs.e[1] && self.e[2] == rhs.e[2]
    }
}


#[test]
fn it_compute_add_of_two_vec3() {
    let v0 = Vec3{e:[1.0,2.0,3.0]};
    let v1 = Vec3{e:[3.0,2.0,1.0]};
    let v2 = Vec3{e:[4.0,4.0,4.0]};
    assert_eq!(v0+v1, v2);
}

#[test]
fn it_compute_lerp_of_two_vec3() {
    let a = Vec3{e:[2.0,4.0,8.0]};
    let b = Vec3{e:[4.0,8.0,16.0]};
    let lerp_0 = Vec3{e:[2.0,4.0,8.0]};
    let lerp_05 = Vec3{e:[3.0,6.0,12.0]};
    let lerp_1 = Vec3{e:[4.0,8.0,16.0]};

    assert_eq!(Vec3::lerp(&a,&b,0.0), lerp_0);
    assert_eq!(Vec3::lerp(&a,&b,0.5), lerp_05);
    assert_eq!(Vec3::lerp(&a,&b,1.0), lerp_1);
}
