mod ray;
mod vec3;
mod hitable;
mod sphere;
mod camera;
mod material;
mod helper;

extern crate rand;

use ray::Ray;
use vec3::{Vec3, ZERO3, ONE3};
use sphere::Sphere;
use hitable::Hitable;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};

use rand::{thread_rng, Rng};

use std::f64;
use std::f64::consts::FRAC_PI_4;
use std::rc::Rc;
use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

fn color(ray: &Ray, world: &Vec<Sphere>, recursion_level: i16) -> Vec3 {
    match world.hit(ray, 0.001, f64::MAX) {
        Some(record) => {
            if recursion_level < 500 {
                match record.material.scatter(ray, &record) {
                    Some((scattered, attenuation)) => {
                        attenuation * color(&scattered, world, recursion_level + 1)
                    }
                    None => ZERO3,
                }
            } else {
                ZERO3
            }
        }
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * ONE3 + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main0() -> Result<(), Error> {
    let mut buffer = try!(File::create("image.ppm"));

    let size_x: u16 = 400;
    let size_y: u16 = 200;
    let samples: u16 = 100;

    let mut world: Vec<Sphere> = Vec::new();

    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0),
                           0.5,
                           Rc::new(Lambertian { albedo: Vec3::new(0.1, 0.2, 0.5) })));
    // world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
    //                        100.0,
    //                        Rc::new(Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) })));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
                           100.0,
                           Rc::new(Metal {
                               albedo: Vec3::new(0.8, 0.8, 0.8),
                               fuzz: 0.0,
                           })));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, -1.0),
                           0.5,
                           Rc::new(Metal {
                               albedo: Vec3::new(0.8, 0.6, 0.2),
                               fuzz: 0.0,
                           })));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                           0.5,
                           Rc::new(Dielectric {
                               reflection_index: 1.8,
                               albedo: Vec3::new(1.0, 1.0, 1.0),
                           })));

    let aspect_ratio = (size_x as f64) / (size_y as f64);

    let look_from = Vec3::new(-1.5, 1.5, 1.5);
    let look_at = Vec3::new(0.0, 0.0, -1.0);

    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.05;

    let camera = Camera::new(look_from,
                             look_at,
                             Vec3::new(0.0, 1.0, 0.0),
                             30.0,
                             aspect_ratio,
                             aperture,
                             dist_to_focus);

    let mut rng = thread_rng();

    try!(write!(buffer, "P3\n{} {}\n255\n", size_x, size_y));

    for j in (0..size_y).rev() {
        for i in 0..size_x {
            let mut col = ZERO3;
            for _i in 0..samples {
                let u = ((i as f64) + rng.gen_range(0.0, 1.0)) / (size_x as f64);
                let v = ((j as f64) + rng.gen_range(0.0, 1.0)) / (size_y as f64);
                let ray = camera.get_ray(u, v);
                col = col + color(&ray, &world, 0);
            }

            let int_r = (255.99 * col.x / (samples as f64)) as u8;
            let int_g = (255.99 * col.y / (samples as f64)) as u8;
            let int_b = (255.99 * col.z / (samples as f64)) as u8;

            try!(write!(buffer, "{} {} {}\n", int_r, int_g, int_b));
        }
    }
    Ok(())
}

fn main() {
    // unwrap() will panic if main0() returns an error.
    main0().unwrap();
}
