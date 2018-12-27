use na::Point3;

use crate::color::*;
use crate::point_light::*;
use crate::ray::*;
use crate::shape::*;
use crate::sphere::*;
use crate::transformation::*;
use crate::types::*;

pub struct Scene {
    pub objects: Vec<Box<Shape>>,
    pub lights: Vec<PointLight>,
}

impl Default for Scene {
    fn default() -> Self {
        let obj = Box::new(Sphere::from(Transformation::new().scale(0.5, 0.5, 0.5)));
        let light = PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            position: Point3::new(-10.0, 10.0, -10.0),
        };
        Self {
            objects: vec![obj],
            lights: vec![light],
        }
    }
}

impl Scene {
    pub fn intersection(&self, ray: &Ray) -> Option<(Float, &Shape)> {
        let mut min_intersection = None;
        for obj in &self.objects {
            match (min_intersection, obj.intersection(ray)) {
                (None, Some(t)) => min_intersection = Some((t, &**obj)),
                (Some((min_t, _)), Some(t)) => {
                    if t < min_t {
                        min_intersection = Some((t, &**obj));
                    }
                }
                _ => {}
            }
        }

        min_intersection
    }
}
