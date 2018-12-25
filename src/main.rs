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
use crate::intersections::*;
use crate::point::*;
use crate::ray::*;
use crate::vector::*;

fn main() {
    let canvas_length = 100;
    let mut image: Image = Vec::with_capacity(canvas_length);
    for i in 0..canvas_length {
        image.push(Vec::with_capacity(canvas_length));
        for _ in 0..canvas_length {
            image[i].push(Color(Vec3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }));
        }
    }

    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / (canvas_length as Float);
    let half = wall_size / 2.0;
    for y in 0..canvas_length {
        let world_y = half - pixel_size * (y as Float);
        for x in 0..canvas_length {
            let world_x = -half + pixel_size * (x as Float);
            let position = v(world_x, world_y, wall_z);
            let r = Ray {
                origin: p(0.0, 0.0, -5.0),
                direction: (position - v(0.0, 0.0, -5.0)).normalize(),
            };
            let color = match ray_sphere_intersection(r) {
                Some((_, _)) => Color(v(1.0, 0.0, 0.0)),
                None => Color(v(0.0, 0.0, 0.0)),
            };

            image[x][y] = color;
        }
    }

    match write_ppm(Path::new("foo.ppm"), &image) {
        Ok(_) => println!("Wrote image!"),
        Err(e) => eprintln!("Could not write image due to:\n{:?}", e),
    };
}
