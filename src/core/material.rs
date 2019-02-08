#![allow(dead_code)]

use crate::core::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterRecord
{
    pub attenuation: Vec3,
    pub scattered: Ray
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}


//
// Lambert material
//

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LambertianMaterial {
    pub albedo: Vec3,
}

impl Material for LambertianMaterial
{
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target-hit_record.p);
        let attenuation = self.albedo;

        Some( ScatterRecord{ attenuation, scattered } )
    }
}

//
// Metal material
//

pub struct MetalMaterial {
    pub albedo: Vec3,
    pub fuzz: f32
}
    

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&normalize(ray.direction), &hit_record.normal);
        let scattered = Ray::new(ray.origin, reflected + self.fuzz*random_in_unit_sphere());
        let attenuation = self.albedo;

        if dot(&scattered.direction, &hit_record.normal) > 0.0 {
            return Some( ScatterRecord{ attenuation, scattered } );
        }

        None
    }
}