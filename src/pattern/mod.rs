mod checkers;
mod gradient;
mod ring;
mod stripe;

use crate::tuple::{Color, Point};
use std::fmt::{Debug, Formatter};

use crate::matrix::Matrix4;
use crate::shape::Shape;

pub use checkers::Checkers;
pub use gradient::LinearGradient;
pub use ring::Ring;
pub use stripe::Stripe;

pub trait Pattern {
    fn color_object(&self, object: &dyn Shape, point: &Point) -> Color {
        let object_point = object.get_inverse_transform() * point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.color_at(&pattern_point)
    }
    fn color_at(&self, point: &Point) -> Color;
    fn get_transform(&self) -> &Matrix4;
    fn set_transform(&mut self, transform: &Matrix4);
}

impl Debug for dyn Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pattern")
    }
}

#[cfg(test)]
#[derive(Debug, Default)]
pub struct TestPattern {
    pub transform: Matrix4,
}

#[cfg(test)]
impl TestPattern {
    pub fn new() -> Box<Self> {
        Box::<TestPattern>::default()
    }
}

#[cfg(test)]
impl Pattern for TestPattern {
    fn color_at(&self, point: &Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }

    fn get_transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = *transform;
    }
}
