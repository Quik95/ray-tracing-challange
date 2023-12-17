use crate::matrix::Matrix4;

use crate::pattern::Pattern;
use crate::tuple::{Color, Point};

#[derive(Debug, Copy, Clone)]
pub struct Stripe {
    pub even: Color,
    pub odd: Color,
    transform: Matrix4,
}

impl Stripe {
    pub fn new(even: Color, odd: Color) -> Box<Self> {
        Box::new(Self {
            even,
            odd,
            transform: Matrix4::identity(),
        })
    }
}

impl Pattern for Stripe {
    fn color_at(&self, point: &Point) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
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
    use crate::matrix::Matrix4;
    use crate::pattern::stripe::Stripe;
    use crate::pattern::Pattern;
    use crate::shape::Sphere;
    use crate::tuple::{Color, Point, Vector};
    use pretty_assertions::assert_eq;

    #[test]
    pub fn stripe_pattern_is_constant_in_y() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 1., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 2., 0.)), Color::white());
    }

    #[test]
    pub fn stripe_pattern_is_constant_in_z() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 1.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 2.)), Color::white());
    }

    #[test]
    pub fn stripe_pattern_alternates_in_x() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(&Point::new(0., 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(0.9, 0., 0.)), Color::white());
        assert_eq!(pattern.color_at(&Point::new(1., 0., 0.)), Color::black());
        assert_eq!(pattern.color_at(&Point::new(-0.1, 0., 0.)), Color::black());
        assert_eq!(pattern.color_at(&Point::new(-1., 0., 0.)), Color::black());
        assert_eq!(pattern.color_at(&Point::new(-1.1, 0., 0.)), Color::white());
    }

    #[test]
    pub fn stripe_with_object_transformation() {
        let obj = Sphere::static_default()
            .set_transform(Matrix4::identity().scale(Vector::new(2., 2., 2.)));
        let pattern = Stripe::new(Color::white(), Color::black());
        let c = pattern.color_object(obj, &Point::new(1.5, 0., 0.));
        assert_eq!(c, Color::white());
    }

    #[test]
    pub fn stripe_with_pattern_transformation() {
        let obj = Sphere::static_default();
        let pattern_transform = Matrix4::identity().scale(Vector::new(2., 2., 2.));
        let mut pattern = Stripe::new(Color::white(), Color::black());
        pattern.set_transform(&pattern_transform);
        let c = pattern.color_object(obj, &Point::new(1.5, 0., 0.));
        assert_eq!(c, Color::white());
    }

    #[test]
    pub fn stripe_with_both_transforms() {
        let obj = Sphere::static_default()
            .set_transform(Matrix4::identity().scale(Vector::new(2., 2., 2.)));
        let mut pattern = Stripe::new(Color::white(), Color::black());
        pattern.set_transform(&Matrix4::identity().translate(Vector::new(0.5, 0., 0.)));
        let c = pattern.color_object(obj, &Point::new(2.5, 0., 0.));
        assert_eq!(c, Color::white());
    }
}
