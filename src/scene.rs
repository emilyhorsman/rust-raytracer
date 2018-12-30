use na::Point3;

use crate::color::*;
use crate::material::*;
use crate::model_transformation::*;
use crate::point_light::*;
use crate::ray::*;
use crate::shape::*;
use crate::sphere::*;
use crate::types::*;

pub struct Scene {
    pub objects: Vec<Box<Shape>>,
    pub lights: Vec<PointLight>,
}

impl Default for Scene {
    fn default() -> Self {
        let obj = Box::new(Sphere {
            object_to_world_space: ModelTransformation::new().matrix(),
            material: Material {
                color: Color::new(1.0, 0.2, 1.0),
                ..Material::default()
            },
        });
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

    pub fn is_occluded(&self, ray: &Ray, distance_threshold: Float) -> bool {
        for obj in &self.objects {
            if let Some(t) = obj.intersection(ray) {
                if t < distance_threshold {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(p: Point3f) -> (Scene, Float, Ray) {
        let scene: Scene = Default::default();
        let (distance, dir) = scene.lights[0].direction_from(&p);
        let shadow_ray = Ray {
            origin: p,
            direction: dir,
        };
        (scene, distance, shadow_ray)
    }

    #[test]
    fn nothing_in_shadow() {
        let (scene, distance, shadow_ray) = setup(Point3::new(0.0, 10.0, 0.0));
        assert!(!scene.is_occluded(&shadow_ray, distance));
    }

    #[test]
    fn in_shadow_when_shape_occludes_point() {
        // Light  Sphere  Point
        let (scene, distance, shadow_ray) = setup(Point3::new(10.0, -10.0, 10.0));
        assert!(scene.is_occluded(&shadow_ray, distance));
    }

    #[test]
    fn not_in_shadow_when_point_behind_light() {
        // Point  Light  Sphere
        let (scene, distance, shadow_ray) = setup(Point3::new(-20.0, 20.0, -20.0));
        assert!(!scene.is_occluded(&shadow_ray, distance));
    }

    #[test]
    fn not_in_shadow_when_object_behind_point() {
        // Light  Point  Sphere
        let (scene, distance, shadow_ray) = setup(Point3::new(-2.0, 20.0, -2.0));
        assert!(!scene.is_occluded(&shadow_ray, distance));
    }
}
