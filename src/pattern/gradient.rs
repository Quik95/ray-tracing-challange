use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use crate::tuple::{Color, Point};

#[derive(Debug, Copy, Clone)]
pub struct LinearGradient {
    start: Color,
    end: Color,
    transform: Matrix4,
    distance: Color,
}

impl LinearGradient {
    pub fn new(start: Color, end: Color) -> Box<Self> {
        Box::new(Self {
            start,
            end,
            transform: Matrix4::identity(),
            distance: end - start,
        })
    }
}

impl Pattern for LinearGradient {
    fn color_at(&self, point: &Point) -> Color {
        let fraction = point.x - point.x.floor();
        if point.x.floor() % 2. == 0. {
            self.start + self.distance * fraction
        } else {
            self.end - self.distance * fraction
        }
    }

    fn get_transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::{LinearGradient, Pattern};
    use crate::tuple::{Color, Point};
    use pretty_assertions::assert_eq;

    #[test]
    pub fn linearly_interpolates_between_colors() {
        let pattern = LinearGradient::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::zero()), Color::white());
        assert_eq!(
            pattern.color_at(&Point::new(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
