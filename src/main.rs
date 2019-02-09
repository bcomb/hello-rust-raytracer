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

    // Sky    
    let t = 0.5 * (ray.direction[1] + 1.0);
    return lerp(&Vec3::new(1.0,1.0,1.0), &Vec3::new(0.5,0.7,1.0), t);
}


fn random_scene() -> Vec<Box<Hitable>> {
    let mut scene : Vec<Box<Hitable>> = Vec::with_capacity(500);
    
    // Ground
    scene.push( Box::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Rc::new(LambertianMaterial { albedo: Vec3::new(0.5,0.5,0.5) }))) );

    // Random
    for a in -11..12 {
        for b in -11..12 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new((a as f32)+0.9*rand::random::<f32>(),0.2,(b as f32)+0.9*rand::random::<f32>());
            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    let mat = Rc::new(LambertianMaterial { albedo: Vec3::new(rand::random::<f32>()*rand::random::<f32>(),rand::random::<f32>()*rand::random::<f32>(),rand::random::<f32>()*rand::random::<f32>()) });
                    let sphere = Box::new(Sphere::new(center, 0.2, mat));
                    scene.push(sphere);
                } else if choose_mat < 0.95 { // metal
                    let albedo = Vec3::new(0.5 * (1.0 + rand::random::<f32>()),0.5 * (1.0 + rand::random::<f32>()),0.5 * (1.0 + rand::random::<f32>()));
                    let fuzz = rand::random::<f32>() * 0.5;
                    let mat = Rc::new(MetalMaterial { albedo, fuzz } );
                    let sphere = Box::new(Sphere::new(center, 0.2, mat));
                    scene.push(sphere);                    
                } else { // glass                    
                    let ref_idx = 1.5;
                    let mat = Rc::new(DielectricMaterial { ref_idx } );
                    let sphere = Box::new(Sphere::new(center, 0.2, mat));
                    scene.push(sphere);
                }                
            }
        }
    }

    // Some big one
    {
        let center = Vec3::new(0.0,1.0,0.0);
        let ref_idx = 1.5;
        let mat = Rc::new(DielectricMaterial { ref_idx } );
        let sphere = Box::new(Sphere::new(center, 1.0, mat));
        scene.push(sphere);
    } 
    {
        let center = Vec3::new(-4.0,1.0,0.0);
        let albedo = Vec3::new(0.4,0.2,0.1);
        let mat = Rc::new(LambertianMaterial { albedo });        
        let sphere = Box::new(Sphere::new(center, 1.0, mat));
        scene.push(sphere);
    }
    {
        let center = Vec3::new(4.0,1.0,0.0);
        let albedo = Vec3::new(0.7,0.6,0.5);
        let fuzz = 0.0;
        let mat = Rc::new(MetalMaterial { albedo, fuzz } );
        let sphere = Box::new(Sphere::new(center, 1.0, mat));
        scene.push(sphere);
    }

    scene
}

fn main() {
    // Scene
    let scene = random_scene();

    // Configuration Camera/Image
    let nx = 512;
    let ny = 512;
    let ns = 1;
    let ratio = nx as f32 / ny as f32;
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(lookfrom, lookat, vup, vfov, ratio, aperture, dist_to_focus);
    
    // PPM header
    println!("P3");
    println!("{} {} 255", nx, ny);
        
    // Start !
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0.0,0.0,0.0);
            for _s in 0..ns {
                let u: f32 = (i as f32 + rand::random::<f32>()) / (nx as f32);
                let v: f32 = (j as f32 + rand::random::<f32>()) / (ny as f32);
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
}
