#[macro_use]
extern crate derive_more;
extern crate nalgebra as na;

mod color;
mod image_output;
mod intersections;
mod material;
mod point_light;
mod ray;
mod shape;
mod sphere;
mod trace;
mod transformation;
mod types;

use std::path::Path;

use na::*;

use crate::color::*;
use crate::image_output::*;
use crate::material::*;
use crate::point_light::*;
use crate::ray::*;
use crate::shape::*;
use crate::sphere::*;
use crate::trace::*;
use crate::transformation::*;
use crate::types::*;

fn main() {
    let canvas_length = 400;
    let mut image: Image = Vec::with_capacity(canvas_length);
    for i in 0..canvas_length {
        image.push(Vec::with_capacity(canvas_length));
        for _ in 0..canvas_length {
            image[i].push(Color::new(0.0, 0.0, 0.0));
        }
    }

    let wall_z = 10.0;
    let wall_size = 6.0;
    let pixel_size = wall_size / (canvas_length as Float);
    let half = wall_size / 2.0;
    let sphere = Sphere::from(
        Transformation::new()
            .translate(1.0, 0.0, 0.0)
            .scale(0.5, 0.5, 0.5),
    );
    let light = PointLight {
        color: Color::new(1.0, 1.0, 1.0),
        position: Point3::new(-10.0, 10.0, -10.0),
    };
    for y in 0..canvas_length {
        let world_y = half - pixel_size * (y as Float);
        for x in 0..canvas_length {
            let world_x = -half + pixel_size * (x as Float);
            let position = Point3::new(world_x, world_y, wall_z);
            let r = Ray {
                origin: Point3::new(0.0, 0.0, -5.0),
                direction: (position - Point3::new(0.0, 0.0, -5.0)).normalize(),
            };
            let color = match sphere.intersection(&r).map(|t| r.from_parameter(t)) {
                Some(intersection_point) => lighting(
                    &Default::default(),
                    &light,
                    &intersection_point,
                    &r,
                    &sphere.normal_at(intersection_point),
                ),
                None => Color::new(0.0, 0.0, 0.0),
            };

            image[x][y] = color;
        }
    }

    match write_ppm(Path::new("foo.ppm"), &image) {
        Ok(_) => println!("Wrote image!"),
        Err(e) => eprintln!("Could not write image due to:\n{:?}", e),
    };
}
