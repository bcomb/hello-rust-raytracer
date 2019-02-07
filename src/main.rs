mod core;
use crate::core::*;


fn main() {
    let v0 = Vec3::new(1.0,2.0,3.0);
    let v1 = Vec3{e:[3.0,2.0,1.0]};
    //let v2 = Vec3{e:[3.0,2.0,1.0]};

    let ray = Ray::new(v0,v1);

    //let mut handle = std::io::stdout().lock();

    let sphere = Sphere::new(v0, 5.0);

    let hit_record = sphere.hit(&ray, 1.0, 5.0);
    match hit_record {
        None => println!("NoIntersection"),
        Some(_) => println!("Intesected !!"),
    }

    // Show the final state of the list
    println!("{:?}", v0 + v1);
    println!("{:?}", ray);
}
