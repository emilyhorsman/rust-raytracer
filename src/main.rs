#[macro_use]
extern crate derive_more;
extern crate nalgebra as na;

mod color;
mod image_output;
mod intersections;
mod material;
mod point_light;
mod ray;
mod scene;
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
use crate::scene::*;
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
    let wall_size = 7.0;
    let pixel_size = wall_size / (canvas_length as Float);
    let half = wall_size / 2.0;
    let mut scene: Scene = Default::default();
    scene.objects.push(Box::new(Sphere::from(
        Transformation::new()
            .scale(0.2, 0.1, 0.2)
            .translate(0.5, 0.0, -2.0),
    )));
    scene.lights.push(PointLight {
        color: Color::new(1.0, 0.0, 0.0),
        position: Point3::new(10.0, 10.0, -10.0),
    });
    for y in 0..canvas_length {
        let world_y = half - pixel_size * (y as Float);
        for x in 0..canvas_length {
            let world_x = -half + pixel_size * (x as Float);
            let position = Point3::new(world_x, world_y, wall_z);
            let r = Ray {
                origin: Point3::new(0.0, 0.0, -5.0),
                direction: (position - Point3::new(0.0, 0.0, -5.0)).normalize(),
            };
            let color = match scene.intersection(&r).map(|(t, obj)| (r.point_at(t), obj)) {
                Some((intersection_point, obj)) => lighting(
                    &Material {
                        color: Color::new(1.0, 0.2, 1.0),
                        ..Default::default()
                    },
                    &scene.lights,
                    &intersection_point,
                    &r,
                    &obj.normal_at(intersection_point),
                )
                .clamp(),
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
