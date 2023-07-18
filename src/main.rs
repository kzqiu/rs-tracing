#![allow(dead_code, unused_variables)]

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::color::{clamp, ray_color};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use std::ops::DerefMut;
use std::rc::Rc;

use image::{Rgb, RgbImage};

// Utility constants and functions
const PI: f64 = 3.1415926535897932385;
const INF: f64 = f64::INFINITY;

fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.;
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new();

    let stride = WIDTH as usize * 3;

    img.deref_mut()
        .par_chunks_exact_mut(stride)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, pixel) in row.chunks_exact_mut(3).enumerate() {
                if x >= WIDTH as usize || y >= HEIGHT as usize {
                    continue;
                }

                let mut color = Vec3::new(0., 0., 0.);

                for s in 0..SAMPLES_PER_PIXEL {
                    let u: f64 = (x as f64 + rand::thread_rng().gen::<f64>()) / (WIDTH as f64 - 1.);
                    let v: f64 = ((HEIGHT - y as u32) as f64 + rand::thread_rng().gen::<f64>())
                        / (HEIGHT as f64 - 1.);

                    let r = camera.get_ray(u, v);
                    color += ray_color(&r, &world);
                }

                let scale = 1. / SAMPLES_PER_PIXEL as f64;

                let a = clamp(color.x() * scale, 0., 0.999);
                let b = clamp(color.y() * scale, 0., 0.999);
                let c = clamp(color.z() * scale, 0., 0.999);

                let Rgb(a) = Rgb([(256. * a) as u8, (256. * b) as u8, (256. * c) as u8]);

                pixel.copy_from_slice(&a);
            }
        });

    match img.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(_) => println!("Done."),
    };
}
