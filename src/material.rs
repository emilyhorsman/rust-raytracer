use crate::color::*;
use crate::pattern::*;
use crate::types::*;

pub struct Material {
    pub color: Box<Pattern>,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Box::new(SolidPattern(Color::new(1.0, 1.0, 1.0))),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
