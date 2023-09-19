use derive_more::{
    Add, AddAssign, Constructor, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::ops::{Add, Sub};

#[derive(
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Neg,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Constructor,
    Debug,
    Copy,
    Clone,
    PartialEq,
)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn zero() -> Self {
        Self::new(0., 0., 0.)
    }
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(
            self.z
                .mul_add(self.z, self.x.mul_add(self.x, self.y * self.y)),
        )
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self::new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.z
            .mul_add(other.z, self.x.mul_add(other.x, self.y * other.y))
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y.mul_add(other.z, -self.z * other.y),
            self.z.mul_add(other.x, -self.x * other.z),
            self.x.mul_add(other.y, -self.y * other.x),
        )
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point> for Vector {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Neg,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Constructor,
    Debug,
    Copy,
    Clone,
    PartialEq,
)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{Point, Vector};
    use test_case::test_case;

    #[test]
    pub fn new_vector() {
        let v = Vector::new(4., -4., 3.);
        assert_eq!(v.x, 4.);
        assert_eq!(v.y, -4.);
        assert_eq!(v.z, 3.);
    }

    #[test]
    pub fn new_point() {
        let v = Point::new(4., -4., 3.);
        assert_eq!(v.x, 4.);
        assert_eq!(v.y, -4.);
        assert_eq!(v.z, 3.);
    }

    #[test]
    pub fn subtracting_a_vector_from_point() {
        let p = Point::new(3., 2., 1.);
        let v = Vector::new(5., 6., 7.);
        let res = p - v;
        assert_eq!(res, Point::new(-2., -4., -6.));
    }

    #[test]
    pub fn subtracting_two_points() {
        let p = Point::new(3., 2., 1.);
        let v = Point::new(5., 6., 7.);
        let res = p - v;
        assert_eq!(res, Point::new(-2., -4., -6.));
    }

    #[test]
    pub fn subtracting_two_vectors() {
        let p = Vector::new(3., 2., 1.);
        let v = Vector::new(5., 6., 7.);
        let res = p - v;
        assert_eq!(res, Vector::new(-2., -4., -6.));
    }

    #[test]
    pub fn subtracting_vector_from_zero_vector() {
        let p = Vector::zero();
        let v = Vector::new(1., -2., 3.);
        let res = p - v;
        assert_eq!(res, Vector::new(-1., 2., -3.));
    }

    #[test]
    pub fn negating_a_vector() {
        let v = Vector::new(1., -2., 3.);
        let res = -v;
        assert_eq!(res, Vector::new(-1., 2., -3.));
    }

    #[test]
    pub fn multiplying_vector_by_scalar() {
        let v = Vector::new(1., -2., 3.);
        let res = v * 3.5;
        assert_eq!(res, Vector::new(3.5, -7., 10.5));
    }

    #[test]
    pub fn multiplying_point_by_scalar() {
        let v = Point::new(1., -2., 3.);
        let res = v * 3.5;
        assert_eq!(res, Point::new(3.5, -7., 10.5));
    }

    #[test]
    pub fn dividing_vector_by_scalar() {
        let v = Vector::new(1., -2., 3.);
        let res = v / 2.;
        assert_eq!(res, Vector::new(0.5, -1., 1.5));
    }

    #[test]
    pub fn dividing_point_by_scalar() {
        let v = Point::new(1., -2., 3.);
        let res = v / 2.;
        assert_eq!(res, Point::new(0.5, -1., 1.5));
    }

    #[test_case(Vector::new(1., 0., 0.), 1.0 ; "when input is (1., 0., 0.)")]
    #[test_case(Vector::new(0., 1., 0.), 1.0 ; "when input is (0., 1., 0.)")]
    #[test_case(Vector::new(0., 0., 1.), 1.0 ; "when input is (0., 0., 1.)")]
    #[test_case(Vector::new(1., 2., 3.), f32::sqrt(14.0) ; "when input is (1., 2., 3.)")]
    #[test_case(Vector::new(-1., -2., -3.), f32::sqrt(14.0) ; "when input is neg((1., 2., 3.))")]
    pub fn vector_magnitude(input: Vector, expected: f32) {
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected);
    }

    #[test_case(Vector::new(4., 0., 0.), Vector::new(1., 0., 0.) ; "when input is (4., 0., 0.)")]
    #[test_case(Vector::new(1., 2., 3.), Vector::new(
        1.0/14.0_f32.sqrt(),
        2.0/14.0_f32.sqrt(),
        3.0/14.0_f32.sqrt(),
    ) ; "when input is (1., 2., 3.)")]
    pub fn normalize_vector(input: Vector, expected: Vector) {
        let normalized = input.normalize();
        assert_eq!(normalized, expected);
    }

    #[test]
    pub fn magnitude_of_normalized_vector_is_one() {
        let mag = Vector::new(1., 2., 3.).normalize().magnitude();
        assert_eq!(mag, 1.0);
    }

    #[test]
    pub fn vector_dot_product() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        let res = a.dot(&b);
        assert_eq!(res, 20.0);
    }

    #[test]
    pub fn vector_cross_product() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        assert_eq!(a.cross(&b), Vector::new(-1., 2., -1.));
        assert_eq!(b.cross(&a), Vector::new(1., -2., 1.));
    }
}
