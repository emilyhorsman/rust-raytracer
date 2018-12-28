#[macro_use]
extern crate derive_more;
extern crate nalgebra as na;

mod camera;
mod color;
mod image_output;
mod intersections;
mod material;
mod model_transformation;
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
use crate::model_transformation::*;
use crate::point_light::*;
use crate::scene::*;
use crate::sphere::*;
use crate::trace::*;
use crate::view_transformation::*;

fn main() {
    let mut scene: Scene = Default::default();
    scene.objects.push(Box::new(Sphere::from(
        ModelTransformation::new()
            .scale(0.2, 0.1, 0.2)
            .translate(0.5, 0.0, -2.0),
    )));
    scene.lights.push(PointLight {
        color: Color::new(0.5, 0.5, 0.5),
        position: Point3::new(10.0, 10.0, -10.0),
    });

    let camera = Camera {
        canvas_width: 500,
        canvas_height: 400,
        field_of_view_radians: FRAC_PI_6,
        transform: ViewTransformation {
            from: Point3::new(0.0, 2.0, -5.0),
            to: Point3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
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
        for x in 0..camera.canvas_width {
            image[x][y] = render(&camera, &scene, x, y);
        }
    }

    match write_ppm(Path::new("foo.ppm"), &image) {
        Ok(_) => println!("Wrote image!"),
        Err(e) => eprintln!("Could not write image due to:\n{:?}", e),
    };
}
