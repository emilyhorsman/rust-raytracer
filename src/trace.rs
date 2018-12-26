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
    let reflection_direction = reflect(&(-shadow_direction), normal);
    let eye = -incoming_ray.direction;
    let reflection_ratio = reflection_direction.dot(&eye);
    let mut specular = Color::new(0.0, 0.0, 0.0);
    if reflection_ratio > 0.0 {
        let factor = reflection_ratio.powf(material.shininess);
        specular = light.color * material.specular * factor;
    }

    (ambient + diffuse + specular).clamp()
}
