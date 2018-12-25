#[macro_use]
extern crate derive_more;
extern crate nearly_eq;

mod color;
mod image_output;
mod intersections;
mod point;
mod ray;
mod vector;

use std::path::Path;

use crate::color::*;
use crate::image_output::*;
use crate::vector::*;

fn main() {
    let u: Vec3f = Vec3 {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    let v: Vec3f = Vec3 {
        x: 1f64,
        y: 2f64,
        z: 3f64,
    };

    println!("u + v = {:?}", u + v);
    println!("u - v = {:?}", u - v);
    println!("u.dot(v) = {:?}", u.dot(v));
    println!("3.0 * v = {:?}", v * 3f64);
    println!("||v|| = {:?}", v.norm());
    println!("after normalize: ||v|| = {:?}", v.normalize().norm());

    let mut image: Image = Vec::with_capacity(100);
    for i in 0..100 {
        image.push(Vec::with_capacity(80));
        for _ in 0..80 {
            image[i].push(Color(Vec3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }));
        }
    }

    for x in 0..100 {
        for y in 0..80 {
            image[x][y] = Color(Vec3f {
                x: (x as f64) / 100.0,
                y: (x as f64) / 100.0,
                z: (x as f64) / 100.0,
            });
        }
    }

    match write_ppm(Path::new("foo.ppm"), &image) {
        Ok(_) => println!("Wrote image!"),
        Err(e) => eprintln!("Could not write image due to:\n{:?}", e),
    };
}
