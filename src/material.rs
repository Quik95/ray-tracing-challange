use crate::pattern::Pattern;
use crate::tuple::Color;
use derive_more::Constructor;
use std::fmt::Debug;

#[derive(Debug, Constructor)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            color: Color::new(1., 1., 1.),
            pattern: None,
        }
    }
}
