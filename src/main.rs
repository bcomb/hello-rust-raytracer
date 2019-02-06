mod core;
use crate::core::*;


fn main() {
    let v0 = Vec3::new(1.0,2.0,3.0);
    let v1 = Vec3{e:[3.0,2.0,1.0]};
    //let v2 = Vec3{e:[3.0,2.0,1.0]};

    let ray = Ray::new(v0,v1);


    // Show the final state of the list
    println!("{:?}", v0 + v1);
    println!("{:?}", ray);
}
