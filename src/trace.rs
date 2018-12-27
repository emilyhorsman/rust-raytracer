use crate::color::*;
use crate::material::*;
use crate::point_light::*;
use crate::ray::*;
use crate::types::*;

pub fn shade_intersection(
    material: &Material,
    lights: &Vec<PointLight>,
    point: &Point3f,
    incoming_ray: &Ray,
    normal: Vec3f,
) -> Color {
    let ambient = material.color * material.ambient;
    let corrected_normal = if incoming_ray.is_inside(&normal) {
        -normal
    } else {
        normal
    };
    ambient
        + lights
            .iter()
            .fold(Color::new(0.0, 0.0, 0.0), |total, light| {
                total + light_contribution(material, light, point, incoming_ray, &corrected_normal)
            })
}

fn light_contribution(
    material: &Material,
    light: &PointLight,
    point: &Point3f,
    incoming_ray: &Ray,
    normal: &Vec3f,
) -> Color {
    let effective_color = material.color.mix(light.color);
    let shadow_direction = light.direction_from(point);

    let facing_ratio = shadow_direction.dot(&normal);
    if facing_ratio < 0.0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let diffuse = effective_color * material.diffuse * facing_ratio;
    let specular = compute_reflection(material, light, incoming_ray, normal, &shadow_direction);
    diffuse + specular
}

/// Returns None if the shadow ray is being reflected away from the eye. Otherwise this
/// returns the cosine of the angle between the reflection vector and eye vector.
fn reflection_ratio(incoming_ray: &Ray, normal: &Vec3f, shadow_direction: &Vec3f) -> Option<Float> {
    let reflection_direction = reflect(&(-shadow_direction), normal);
    let ratio = reflection_direction.dot(&incoming_ray.direction_to_origin());
    if ratio <= 0.0 {
        None
    } else {
        Some(ratio)
    }
}

fn compute_reflection(
    material: &Material,
    light: &PointLight,
    incoming_ray: &Ray,
    normal: &Vec3f,
    shadow_direction: &Vec3f,
) -> Color {
    let ratio = match reflection_ratio(incoming_ray, normal, shadow_direction) {
        Some(r) => r,
        None => return Color::new(0.0, 0.0, 0.0),
    };

    let factor = ratio.powf(material.shininess);
    light.color * material.specular * factor
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;

    use approx::*;
    use na::{Point3, Vector3};

    use super::*;
    use crate::shape::*;
    use crate::sphere::*;
    use crate::transformation::*;

    #[test]
    fn it_computes_lighting_behind_eye() {
        let light = PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            position: Point3::new(0.0, 0.0, -10.0),
        };
        let r = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let color = shade_intersection(
            &Default::default(),
            &vec![light],
            &Point3::new(0.0, 0.0, 0.0),
            &r,
            Vector3::new(0.0, 0.0, -1.0),
        );
        assert_relative_eq!(color.0, Vector3::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn it_computes_lighting_with_specular_falloff() {
        let light = PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            position: Point3::new(0.0, 0.0, -10.0),
        };
        // The normal is pointing directly at the light source with the eye observing from
        // a 45 degree offset above and between the point and light. There should be zero
        // specular contribution.
        let r = Ray {
            origin: Point3::new(0.0, 5.0, -5.0),
            direction: Vector3::new(0.0, FRAC_PI_4.sin(), FRAC_PI_4.sin()),
        };
        let color = shade_intersection(
            &Default::default(),
            &vec![light],
            &Point3::new(0.0, 0.0, 0.0),
            &r,
            Vector3::new(0.0, 0.0, -1.0),
        );
        assert_relative_eq!(color.0, Vector3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn it_computes_lighting_with_diffuse_falloff() {
        let light = PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            // 45 degree offset from point.
            position: Point3::new(0.0, 10.0, -10.0),
        };
        // The normal is pointing directly at the eye but 45 degrees from the light source.
        let r = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let color = shade_intersection(
            &Default::default(),
            &vec![light],
            &Point3::new(0.0, 0.0, 0.0),
            &r,
            Vector3::new(0.0, 0.0, -1.0),
        );
        let k = 0.1 + 0.9 * FRAC_PI_4.sin();
        assert_relative_eq!(color.0, Vector3::new(k, k, k));
    }

    #[test]
    fn it_computes_lighting_with_diffuse_falloff_and_eye_in_reflection() {
        let light = PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            // 45 degree offset from point.
            position: Point3::new(0.0, 10.0, -10.0),
        };
        // The light source and eye are 90 degrees apart and each are 45 degrees from the
        // normal.
        let r = Ray {
            origin: Point3::new(0.0, -10.0, -10.0),
            direction: Vector3::new(0.0, FRAC_PI_4.sin(), FRAC_PI_4.sin()),
        };
        let color = shade_intersection(
            &Default::default(),
            &vec![light],
            &Point3::new(0.0, 0.0, 0.0),
            &r,
            Vector3::new(0.0, 0.0, -1.0),
        );
        let k = 0.1 + 0.9 + 0.9 * FRAC_PI_4.sin();
        // Unfortunately this test suffers a liiiiitle more floating point error than
        // usual.
        assert!(relative_eq!(
            color.0,
            Vector3::new(k, k, k),
            epsilon = 1e-13
        ));
    }

    #[test]
    fn it_computes_lighting_behind_surface() {
        let light = PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            // The light is positioned behind the point.
            position: Point3::new(0.0, 0.0, 10.0),
        };
        let r = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let color = shade_intersection(
            &Default::default(),
            &vec![light],
            &Point3::new(0.0, 0.0, 0.0),
            &r,
            Vector3::new(0.0, 0.0, -1.0),
        );
        assert_relative_eq!(color.0, Vector3::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn it_computes_lighting_inside_object() {
        let light = PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            position: Point3::new(0.0, 0.0, 0.0),
        };
        let r = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let s = Sphere::from(Transformation::new());
        let point = s.intersection(&r).map(|t| r.point_at(t)).unwrap();
        let color = shade_intersection(
            &Default::default(),
            &vec![light],
            &point,
            &r,
            s.normal_at(point),
        );
        assert_relative_eq!(color.0, Vector3::new(1.9, 1.9, 1.9));
    }
}
