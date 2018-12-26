use crate::color::*;
use crate::types::*;

pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 0.2, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 20.0,
        }
    }
}
