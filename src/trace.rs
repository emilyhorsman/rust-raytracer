use crate::color::*;
use crate::material::*;
use crate::point_light::*;
use crate::ray::*;
use crate::types::*;

pub fn lighting(
    material: Material,
    light: &PointLight,
    point: Point3f,
    eye: Vec3f,
    normal: Vec3f,
) -> Color {
    let effective_color = material.color.mix(light.color);
    let light_vector = (light.position - point).normalize();
    let ambient = effective_color * material.ambient;

    let facing_ratio = light_vector.dot(&normal);
    if facing_ratio < 0.0 {
        return ambient;
    }

    let diffuse = effective_color * material.diffuse * facing_ratio;
    let reflection_direction = reflect(-light_vector, normal);
    let reflection_ratio = reflection_direction.dot(&eye);
    let mut specular = Color::new(0.0, 0.0, 0.0);
    if reflection_ratio > 0.0 {
        let factor = reflection_ratio.powf(material.shininess);
        specular = light.color * material.specular * factor;
    }

    (ambient + diffuse + specular).clamp()
}
