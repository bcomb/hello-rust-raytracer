#![allow(dead_code)]

extern crate rand;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn inv_length(&self) -> f32 {
        1.0 / self.squared_length().sqrt()
    }

    pub fn normalize(&mut self) {
        let k = self.inv_length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

//
// Add traits
//

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]] }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]] }
    }
}

//
// Sub traits
//
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] }
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] }
    }
}

//
// Mul traits
//
impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]] }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 { e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs] }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]] }
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 { e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]] }
    }
}

//
// Div traits
//
impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2]] }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 { e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs] }
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self / rhs.e[0], self / rhs.e[1], self / rhs.e[2]] }
    }
}

impl ops::Div<&Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Vec3 {
        Vec3 { e: [self / rhs.e[0], self / rhs.e[1], self / rhs.e[2]] }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

//
// [] index access traits
//
impl ops::Index<usize> for Vec3 {
  type Output = f32;

  fn index(&self, i: usize) -> &f32 {
    &self.e[i]
  }
}

impl ops::IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, i: usize) -> &mut f32 {
    &mut self.e[i]
  }
}

//
// Global utility function
//


pub fn normalize(rhs: Vec3) -> Vec3 {
    rhs * rhs.inv_length()
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 { e: [
                v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
                v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0]
                ]}
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

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    let ref k = 2.0*dot(v,n)*n;
    v - k
}

pub fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    if dot(&p,&p) < 1.0 {
      return p;
    }
  }
}

pub fn random_in_unit_sphere() -> Vec3 {
  loop {
    let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
    if dot(&p,&p) < 1.0 {
      return p;
    }
  }
}

//
// Unit test
//
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
