#![allow(dead_code)]

use crate::core::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterRecord
{
    pub attenuation: Vec3,
    pub scattered: Ray
}

pub trait Material : std::fmt::Debug {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetalMaterial {
    pub albedo: Vec3,
    pub fuzz: f32
}
    
impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(ray.origin, reflected + self.fuzz*random_in_unit_sphere());
        let attenuation = self.albedo;

        if dot(&scattered.direction, &hit_record.normal) > 0.0 {
            return Some( ScatterRecord{ attenuation, scattered } );
        }

        None
    }
}

//
// Dielectric material
//
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0sq = r0 * r0;
    r0sq + (1.0 - r0sq) * (1.0 - cosine).powf(5.0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DielectricMaterial {
  pub ref_idx: f32
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let outward_normal : Vec3;
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let ni_over_nt;
        
        let cosine : f32;
        let n_dot_l = dot(&ray.direction, &hit_record.normal);
        if  n_dot_l > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
            cosine = (1.0 - self.ref_idx*self.ref_idx*(1.0-n_dot_l*n_dot_l)).sqrt();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -n_dot_l;
        }

        if let Some(refracted) = refract(&ray.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);

            if rand::random::<f32>() < reflect_prob {
                return Some( ScatterRecord { scattered: Ray::new(hit_record.p, reflected), attenuation: Vec3::new(1.0,1.0,1.0) }  );
            }
            else
            {
                return Some( ScatterRecord { scattered: Ray::new(hit_record.p, refracted), attenuation: Vec3::new(1.0,1.0,1.0) }  );
            }            
        } else {
            return Some( ScatterRecord { scattered: Ray::new(hit_record.p, reflected), attenuation: Vec3::new(1.0,1.0,1.0) }  );
        }
    }
}
