use derive_more::{
    Add, AddAssign, Constructor, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use nalgebra::Point4;
use std::ops::{Add, Mul, Sub};

pub const EPSILON: f32 = 0.00001;

pub fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
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
    Default,
)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn zero() -> Self {
        Self::default()
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

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2. * self.dot(normal)
    }
}

impl From<nalgebra::Vector4<f32>> for Vector {
    fn from(value: nalgebra::Vector4<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector> for nalgebra::Vector4<f32> {
    fn from(val: Vector) -> Self {
        Self::new(val.x, val.y, val.z, 0.)
    }
}

impl Eq for Vector {}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
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

#[derive(Neg, Mul, MulAssign, Div, DivAssign, Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub const fn one() -> Self {
        Self::new(1., 1., 1.)
    }
}

impl Add for Point {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

impl From<Point> for nalgebra::Point4<f32> {
    fn from(val: Point) -> Self {
        Self::new(val.x, val.y, val.z, 1.0)
    }
}

impl From<nalgebra::Point4<f32>> for Point {
    fn from(value: Point4<f32>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
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

impl Sub<&Self> for Point {
    type Output = Vector;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Constructor,
    Debug,
    Copy,
    Clone,
    Default,
)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn hadamard_product(&self, other: &Self) -> Self {
        Self::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }

    pub const fn white() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
    pub const fn black() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }
}

impl Eq for Color {}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.r, other.r) && approx_eq(self.g, other.g) && approx_eq(self.b, other.b)
    }
}

impl Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.hadamard_product(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{approx_eq, Color, Point, Vector};
    use pretty_assertions::assert_eq;
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
        assert_eq!(res, Vector::new(-2., -4., -6.));
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

    #[test_case(Vector::new(1., 0., 0.), 1.0; "when input is (1., 0., 0.)")]
    #[test_case(Vector::new(0., 1., 0.), 1.0; "when input is (0., 1., 0.)")]
    #[test_case(Vector::new(0., 0., 1.), 1.0; "when input is (0., 0., 1.)")]
    #[test_case(Vector::new(1., 2., 3.), f32::sqrt(14.0); "when input is (1., 2., 3.)")]
    #[test_case(Vector::new(- 1., - 2., - 3.), f32::sqrt(14.0); "when input is neg((1., 2., 3.))")]
    pub fn vector_magnitude(input: Vector, expected: f32) {
        let magnitude = input.magnitude();
        assert_eq!(magnitude, expected);
    }

    #[test_case(Vector::new(4., 0., 0.), Vector::new(1., 0., 0.); "when input is (4., 0., 0.)")]
    #[test_case(Vector::new(1., 2., 3.), Vector::new(
    1.0 / 14.0_f32.sqrt(),
    2.0 / 14.0_f32.sqrt(),
    3.0 / 14.0_f32.sqrt(),
    ); "when input is (1., 2., 3.)")]
    pub fn normalize_vector(input: Vector, expected: Vector) {
        let normalized = input.normalize();
        assert_eq!(normalized, expected);
    }

    #[test]
    pub fn magnitude_of_normalized_vector_is_one() {
        let mag = Vector::new(1., 2., 3.).normalize().magnitude();
        assert!(approx_eq(mag, 1.));
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

    #[test]
    pub fn adding_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        assert_eq!(a + b, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    pub fn subtracting_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        assert_eq!(a - b, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    pub fn multiplying_color_by_scalar() {
        let a = Color::new(0.2, 0.3, 0.4);
        assert_eq!(a * 2., Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    pub fn multiplying_colors() {
        let a = Color::new(1., 0.2, 0.4);
        let b = Color::new(0.9, 1., 0.1);
        assert_eq!(a * b, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    pub fn reflect_at_45_degree() {
        let v = Vector::new(1., -1., 0.);
        let n = Vector::new(0., 1., 0.);
        let r = v.reflect(&n);
        assert_eq!(r, Vector::new(1., 1., 0.));
    }

    #[test]
    pub fn reflect_off_slanted_surface() {
        let v = Vector::new(0., -1., 0.);
        let n = Vector::new(2_f32.sqrt() / 2., 2_f32.sqrt() / 2., 0.);
        let r = v.reflect(&n);
        assert_eq!(r, Vector::new(1., 0., 0.));
    }
}
