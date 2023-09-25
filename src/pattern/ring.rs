use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use crate::tuple::{Color, Point};

#[derive(Debug, Copy, Clone)]
pub struct Ring {
    even: Color,
    odd: Color,
    transform: Matrix4,
}

impl Ring {
    pub fn new(even: Color, odd: Color) -> Box<Self> {
        Box::new(Self {
            even,
            odd,
            transform: Matrix4::identity(),
        })
    }
}

impl Pattern for Ring {
    fn color_at(&self, point: &Point) -> Color {
        if point.x.hypot(point.z).floor() % 2. == 0. {
            self.even
        } else {
            self.odd
        }
    }

    fn get_transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = *transform;
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::{Pattern, Ring};
    use crate::tuple::{Color, Point};

    #[test]
    pub fn ring_should_extend_in_both_x_and_z() {
        let pattern = Ring::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(1., 0., 0.)), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 1.)), Color::black());
        assert_eq!(
            pattern.color_at(&Point::new(0.708, 0., 0.708)),
            Color::black()
        );
    }
}
