#![allow(clippy::unreadable_literal)]
#[macro_use]
extern crate derive_more;
extern crate nalgebra as na;

mod camera;
mod color;
mod image_output;
mod intersections;
mod material;
mod model_transformation;
mod plane;
mod point_light;
mod ray;
mod scene;
mod shape;
mod sphere;
mod trace;
mod types;
mod view_transformation;

use std::f64::consts::*;
use std::path::Path;

use na::*;

use crate::camera::*;
use crate::color::*;
use crate::image_output::*;
use crate::material::*;
use crate::plane::*;
use crate::point_light::*;
use crate::scene::*;
use crate::trace::*;
use crate::types::*;
use crate::view_transformation::*;

fn make_standard_material(r: Float, g: Float, b: Float) -> Material {
    Material {
        color: Color::new(r, g, b),
        ..Default::default()
    }
}

fn main() {
    let mut scene = Scene {
        objects: vec![],
        lights: vec![],
    };

    scene.objects.push(Box::new(Plane::floor(
        -3.0,
        make_standard_material(0.454902, 0.72549, 1.0),
    )));
    scene.objects.push(Box::new(Plane::left_wall(
        -3.0,
        make_standard_material(0.0, 0.721569, 0.580392),
    )));
    scene.objects.push(Box::new(Plane::right_wall(
        3.0,
        make_standard_material(1.0, 0.917647, 0.654902),
    )));
    scene.objects.push(Box::new(Plane::back_wall(
        2.0,
        make_standard_material(0.333333, 0.937255, 0.768627),
    )));
    scene.objects.push(Box::new(Plane::ceiling(
        3.0,
        make_standard_material(0.882353, 0.439216, 0.333333),
    )));

    scene.lights.push(PointLight {
        color: Color::new(0.5, 0.5, 0.5),
        position: Point3::new(0.0, 1.5, -2.0),
    });

    let camera = Camera {
        canvas_width: 500,
        canvas_height: 400,
        field_of_view_radians: FRAC_PI_2,
        transform: ViewTransformation {
            from: Point3::new(0.0, 0.0, -5.0),
            to: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::y(),
        }
        .matrix(),
    };

    let mut image: Image = Vec::with_capacity(camera.canvas_width);
    for i in 0..camera.canvas_width {
        image.push(Vec::with_capacity(camera.canvas_height));
        for _ in 0..camera.canvas_height {
            image[i].push(Color::new(0.0, 0.0, 0.0));
        }
    }

    for y in 0..camera.canvas_height {
        #[allow(clippy::needless_range_loop)]
        for x in 0..camera.canvas_width {
            image[x][y] = render(&camera, &scene, x, y);
        }
    }

    match write_ppm(Path::new("foo.ppm"), &image) {
        Ok(_) => println!("Wrote image!"),
        Err(e) => eprintln!("Could not write image due to:\n{:?}", e),
    };
}
