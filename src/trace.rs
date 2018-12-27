use crate::color::*;
use crate::material::*;
use crate::point_light::*;
use crate::ray::*;
use crate::types::*;

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Point3f,
    incoming_ray: &Ray,
    normal: &Vec3f,
) -> Color {
    let effective_color = material.color.mix(light.color);
    let shadow_direction = light.direction_from(point);
    let ambient = effective_color * material.ambient;

    let facing_ratio = shadow_direction.dot(&normal);
    if facing_ratio < 0.0 {
        return ambient;
    }

    let diffuse = effective_color * material.diffuse * facing_ratio;
    let specular = compute_reflection(material, light, incoming_ray, normal, &shadow_direction);

    (ambient + diffuse + specular).clamp()
}

/// Returns None if the shadow ray is being reflected away from the eye. Otherwise this
/// returns the cosine of the angle between the reflection vector and eye vector.
fn reflection_ratio(incoming_ray: &Ray, normal: &Vec3f, shadow_direction: &Vec3f) -> Option<Float> {
    let reflection_direction = reflect(&(-shadow_direction), normal);
    let ratio = reflection_direction.dot(&(-incoming_ray.direction));
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
