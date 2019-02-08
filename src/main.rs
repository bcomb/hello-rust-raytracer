extern crate rand;
mod core;
use crate::core::*;
use std::rc::Rc;
use std::f32;

fn compute_color(scene: &Vec<Box<Hitable>>, ray: &Ray, depth: i32) -> Vec3 {

    if let Some(hit_record) = closest_hit(&scene, ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(scatter_record) = hit_record.material.scatter(&ray, &hit_record) {
                return scatter_record.attenuation * compute_color(&scene, &scatter_record.scattered, depth+1);
            }
            else
            {
                return Vec3::new(0.0,0.0,0.0);
            }
        }
    }
    
    let t = 0.5 * (ray.direction[1] + 1.0);
    return lerp(&Vec3::new(1.0,1.0,1.0), &Vec3::new(0.5,0.7,1.0), t);
}

fn main() {
    // Scene
    let lambert_red_mat = Rc::new(LambertianMaterial { albedo: Vec3::new(1.0,0.0,0.0) });
    let scene : Vec<Box<Hitable>> = vec![Box::new(Sphere::new(Vec3::new(0.0,0.0,5.0), 1.0, lambert_red_mat))];

    // Configuration Camera/Image
    let nx = 512;
    let ny = 512;
    let ns = 1;
    let ratio = nx as f32 / ny as f32;
    let lookfrom = Vec3::new(0.0,0.0,0.0);
    let lookat = Vec3::new(0.0,0.0,1.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let vfov = 45.0;
    let dist_to_focus = 4.0;
    let aperture = 0.1;
    let camera = Camera::new(lookfrom, lookat, vup, vfov, ratio, aperture, dist_to_focus);

    let ray = camera.get_ray(0.5,0.5);
    let col = compute_color(&scene, &ray, 0);


    // PPM header
    println!("P3");
    println!("{} {} 255", nx, ny);
    
        
    // Start !
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0.0,0.0,0.0);
            for _s in 0..ns {
                //let u: f32 = (i as f32 + rand::random::<f32>()) / (nx as f32);
                //let v: f32 = (j as f32 + rand::random::<f32>()) / (ny as f32);
                let u: f32 = (i as f32) / (nx as f32);
                let v: f32 = (j as f32) / (ny as f32);                
                let ray = camera.get_ray(u,v);
                color += compute_color(&scene, &ray, 0);
            }
            color /= ns as f32;
            color = Vec3::new( color[0].sqrt(),color[1].sqrt(),color[2].sqrt() );
            color *= 255.99;

            // Output rgb
            println!("{} {} {}", color[0] as u8, color[1] as u8, color[2] as u8);
        }
    }

    // Show the final state of the list
    //println!("{:?}", v0 + v1);
    //println!("{:?}", ray);
}
