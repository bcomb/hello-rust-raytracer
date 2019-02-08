mod core;
use crate::core::*;


fn main() {

    // Scene
    let lambert_mat = Box::new(LambertianMaterial { albedo: Vec3::new(1.0,0.0,0.0) });
    let sphere = Sphere::new(Vec3::new(0.0,0.0,5.0), 1.0, lambert_mat);

    let ray = Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,1.0));
    
    let hit_record = sphere.hit(&ray, 0.0, 10.0);
    match hit_record {
        None => println!("[NoHit]]"),
        Some(_) => println!("[Hit] {:?}", hit_record),
    }

    // Show the final state of the list
    //println!("{:?}", v0 + v1);
    //println!("{:?}", ray);
}
