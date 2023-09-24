use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use crate::tuple::{Color, Point};

#[derive(Debug, Copy, Clone)]
pub struct Checkers {
    even: Color,
    odd: Color,
    transform: Matrix4,
}

impl Checkers {
    pub fn new(even: Color, odd: Color) -> Box<Self> {
        Box::new(Self {
            even,
            odd,
            transform: Matrix4::identity(),
        })
    }
}

impl Pattern for Checkers {
    fn color_at(&self, point: &Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i32 % 2 == 0 {
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
    use crate::pattern::{Checkers, Pattern};
    use crate::tuple::{Color, Point};

    #[test]
    pub fn checkers_should_repeat_in_x() {
        let pattern = Checkers::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0.99, 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(1.01, 0., 0.)), Color::black());
    }

    #[test]
    pub fn checkers_should_repeat_in_y() {
        let pattern = Checkers::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 0.99, 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 1.01, 0.)), Color::black());
    }

    #[test]
    pub fn checkers_should_repeat_in_z() {
        let pattern = Checkers::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.99)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 1.01)), Color::black());
    }
}
