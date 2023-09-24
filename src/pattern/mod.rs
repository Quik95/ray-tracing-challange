mod checkers;
mod gradient;
mod ring;
mod stripe;

use crate::tuple::{Color, Point};
use std::fmt::{Debug, Formatter};

use crate::matrix::Matrix4;
use crate::objects::Shape;

pub use checkers::Checkers;
pub use gradient::LinearGradient;
pub use ring::Ring;
pub use stripe::Stripe;

pub trait Pattern {
    fn color_object(&self, object: &dyn Shape, point: &Point) -> Color {
        let object_point = object.get_transform().inverse() * point;
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
