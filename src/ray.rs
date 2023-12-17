use crate::matrix;

use crate::tuple::{Point, Vector};
use derive_more::Constructor;

#[derive(Debug, Constructor, Copy, Clone, Eq, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(self, matrix: &matrix::Matrix4) -> Self {
        Self::new(matrix * self.origin, matrix * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::tuple::{Point, Vector};
    use pretty_assertions::assert_eq;

    #[test]
    pub fn creating_ray() {
        let origin = crate::tuple::Point::new(1., 2., 3.);
        let direction = crate::tuple::Vector::new(4., 5., 6.);
        let r = crate::ray::Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    pub fn computing_point_from_distance() {
        let r = Ray::new(
            crate::tuple::Point::new(2., 3., 4.),
            crate::tuple::Vector::new(1., 0., 0.),
        );
        assert_eq!(r.position(0.), crate::tuple::Point::new(2., 3., 4.));
        assert_eq!(r.position(1.), crate::tuple::Point::new(3., 3., 4.));
        assert_eq!(r.position(-1.), crate::tuple::Point::new(1., 3., 4.));
        assert_eq!(r.position(2.5), crate::tuple::Point::new(4.5, 3., 4.));
    }

    #[test]
    pub fn translating_ray() {
        let r = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let t = Matrix4::identity().translate(Vector::new(3., 4., 5.));
        let r2 = r.transform(&t);
        assert_eq!(r2.origin, Point::new(4., 6., 8.));
        assert_eq!(r2.direction, Vector::new(0., 1., 0.));
    }

    #[test]
    pub fn scaling_ray() {
        let r = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let t = Matrix4::identity().scale(Vector::new(2., 3., 4.));
        let r2 = r.transform(&t);
        assert_eq!(r2.origin, Point::new(2., 6., 12.));
        assert_eq!(r2.direction, Vector::new(0., 3., 0.));
    }
}
