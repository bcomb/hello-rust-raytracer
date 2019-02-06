mod vec3;

fn main() {
    let v0 = vec3::Vec3::new(1.0,2.0,3.0);
    let v1 = vec3::Vec3{e:[3.0,2.0,1.0]};
    //let v2 = Vec3{e:[3.0,2.0,1.0]};


    // Show the final state of the list
    println!("{:?}", v0 + v1);
}
