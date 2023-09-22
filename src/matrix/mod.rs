use crate::tuple::{approx_eq, Point, Vector};
use std::ops::Mul;

use nalgebra::{matrix, Point4, Vector4};

#[derive(Copy, Clone, Debug)]
pub struct Matrix4(nalgebra::Matrix4<f32>);

impl Eq for Matrix4 {}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .data
            .as_slice()
            .iter()
            .zip(other.0.data.as_slice().iter())
            .all(|(x, y)| approx_eq(*x, *y))
    }
}

impl From<nalgebra::Matrix4<f32>> for Matrix4 {
    fn from(value: nalgebra::Matrix4<f32>) -> Self {
        Self(value)
    }
}

impl From<Matrix4> for nalgebra::Matrix4<f32> {
    fn from(val: Matrix4) -> Self {
        val.0
    }
}

impl Mul<nalgebra::Matrix4<f32>> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: nalgebra::Matrix4<f32>) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let p: Point4<f32> = rhs.into();
        let res = self.0 * p;
        Point::new(res.x, res.y, res.z)
    }
}

impl Mul<Vector> for Matrix4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let p: Vector4<f32> = rhs.into();
        let res = self.0 * p;
        Vector::new(res.x, res.y, res.z)
    }
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self(nalgebra::Matrix4::identity())
    }

    pub fn translate(self, translation: &Vector) -> Self {
        let t = nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(
            translation.x,
            translation.y,
            translation.z,
        ));
        Self(t * self.0)
    }

    pub fn scale(self, scale: &Vector) -> Self {
        let t = nalgebra::Matrix4::new_nonuniform_scaling(&nalgebra::Vector3::new(
            scale.x, scale.y, scale.z,
        ));
        Self(t * self.0)
    }

    fn rotate(self, axis: &nalgebra::Unit<nalgebra::Vector3<f32>>, angle: f32) -> Self {
        let t = nalgebra::Matrix4::from_axis_angle(axis, angle);
        Self(t * self.0)
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        self.rotate(
            &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(1., 0., 0.)),
            angle,
        )
    }

    pub fn rotate_y(self, angle: f32) -> Self {
        self.rotate(
            &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 1., 0.)),
            angle,
        )
    }

    pub fn rotate_z(self, angle: f32) -> Self {
        self.rotate(
            &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 0., 1.)),
            angle,
        )
    }

    pub fn transpose(self) -> Self {
        Self(self.0.transpose())
    }

    pub fn inverse(self) -> Self {
        Self(self.0.try_inverse().unwrap())
    }

    pub fn shear(self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        Self(
            matrix![
                1., xy, xz, 0.;
                yx, 1., yz, 0.;
                zx, zy, 1., 0.;
                0., 0., 0., 1.
            ] * self.0,
        )
    }
}

impl Vector {
    pub fn scale(self, scale: &Vector) -> Self {
        let t = Matrix4::identity().scale(scale);
        t * self
    }
}

impl Point {
    pub fn translate(self, translation: &Vector) -> Self {
        let t = Matrix4::identity().translate(translation);
        t * self
    }

    pub fn scale(self, scale: &Vector) -> Self {
        let t = Matrix4::identity().scale(scale);
        t * self
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        Matrix4::identity().rotate_x(angle) * self
    }

    pub fn rotate_y(self, angle: f32) -> Self {
        Matrix4::identity().rotate_y(angle) * self
    }

    pub fn rotate_z(self, angle: f32) -> Self {
        Matrix4::identity().rotate_z(angle) * self
    }

    pub fn shear(self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let t = Matrix4::identity().shear(xy, xz, yx, yz, zx, zy);
        t * self
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::tuple::{Point, Vector};
    use nalgebra::matrix;
    use std::f32::consts::PI;
    use test_case::test_case;

    #[test]
    pub fn comparing_identical_matrix4() {
        let a: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ]
        .into();
        let b: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ]
        .into();

        assert_eq!(a, b);
    }

    #[test]
    pub fn comparing_different_matrix4() {
        let a: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ]
        .into();
        let b: Matrix4 = matrix![
            2., 3., 4., 5.;
            6., 7., 8., 9.;
            8., 7., 6., 5.;
            4., 3., 2., 1.
        ]
        .into();

        assert_ne!(a, b);
    }

    #[test]
    pub fn multiplying_matrices() {
        let a: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ]
        .into();
        let b: Matrix4 = matrix![
            -2., 1., 2., 3.;
            3., 2., 1., -1.;
            4., 3., 6., 5.;
            1., 2., 7., 8.
        ]
        .into();

        let res: Matrix4 = matrix![
            20., 22., 50., 48.;
            44., 54., 114., 108.;
            40., 58., 110., 102.;
            16., 26., 46., 42.
        ]
        .into();

        assert_eq!(a * b, res);
    }

    #[test]
    pub fn multiply_by_point() {
        let A: Matrix4 = matrix![
            1., 2., 3., 4.;
            2., 4., 4., 2.;
            8., 6., 4., 1.;
            0., 0., 0., 1.
        ]
        .into();
        let b = Point::new(1., 2., 3.);
        let res: Point = A * b;

        assert_eq!(res, Point::new(18., 24., 33.));
    }

    #[test]
    pub fn multiplying_matrix_by_identity_matrix() {
        let a: Matrix4 = matrix![
            0., 1., 2., 4.;
            1., 2., 4., 8.;
            2., 4., 8., 16.;
            4., 8., 16., 32.
        ]
        .into();
        let b = Matrix4::identity();
        assert_eq!(a * b, a);
    }

    #[test]
    pub fn multiplying_point_by_identity_matrix() {
        let a = Point::new(1., 2., 3.);
        let b = Matrix4::identity();
        assert_eq!(b * a, a);
    }

    #[test]
    pub fn transposing_matrix() {
        let a: Matrix4 = matrix![
            0., 9., 3., 0.;
            9., 8., 0., 8.;
            1., 8., 5., 3.;
            0., 0., 5., 8.
        ]
        .into();
        let transposed: Matrix4 = matrix![
            0., 9., 1., 0.;
            9., 8., 8., 0.;
            3., 0., 5., 5.;
            0., 8., 3., 8.
        ]
        .into();

        assert_eq!(a.transpose(), transposed);
    }

    #[test]
    pub fn transposing_identity_matrix() {
        let a = Matrix4::identity();
        assert_eq!(a.transpose(), a);
    }

    #[test]
    pub fn inverse_matrix() {
        let a: Matrix4 = matrix![
            3., -9., 7., 3.;
            3., -8., 2., -9.;
            -4., 4., 4., 1.;
            -6., 5., -1., 1.
        ]
        .into();
        let b: Matrix4 = matrix![
            8., 2., 2., 2.;
            3., -1., 7., 0.;
            7., 0., 5., 4.;
            6., -2., 0., 5.
        ]
        .into();
        let c = a * b;
        let res = c * b.inverse();
        assert_eq!(res, a);
    }

    #[test]
    pub fn translate_point() {
        let p = Point::new(5., -3., 2.).translate(&Vector::new(-3., 4., 5.));
        assert_eq!(p, Point::new(2., 1., 7.));
    }

    #[test]
    pub fn inverse_undoes_translation() {
        let t = Matrix4::identity().translate(&Vector::new(5., -3., 2.));
        let p = Point::new(-3., 4., 5.);
        let res = t.inverse() * p;
        assert_eq!(res, Point::new(-8., 7., 3.));
    }

    #[test]
    pub fn scaling_point() {
        let p = Point::new(-4., 6., 8.).scale(&Vector::new(2., 3., 4.));
        assert_eq!(p, Point::new(-8., 18., 32.));
    }

    #[test]
    pub fn inverse_undoes_scale_point() {
        let t = Matrix4::identity().scale(&Vector::new(2., 3., 4.));
        let p = Point::new(-4., 6., 8.);
        let res = t.inverse() * p;
        assert_eq!(res, Point::new(-2., 2., 2.));
    }

    #[test]
    pub fn scaling_vector() {
        let p = Vector::new(-4., 6., 8.).scale(&Vector::new(2., 3., 4.));
        assert_eq!(p, Vector::new(-8., 18., 32.));
    }

    #[test]
    pub fn inverse_undoes_scale_vector() {
        let t = Matrix4::identity().scale(&Vector::new(2., 3., 4.));
        let p = Vector::new(-4., 6., 8.);
        let res = t.inverse() * p;
        assert_eq!(res, Vector::new(-2., 2., 2.));
    }

    #[test]
    pub fn rotating_point_around_x() {
        let p = Point::new(0., 1., 0.);
        let res1 = p.rotate_x(PI / 4.);
        let res2 = p.rotate_x(PI / 2.);

        assert_eq!(res1, Point::new(0., 2_f32.sqrt() / 2., 2_f32.sqrt() / 2.));
        assert_eq!(res2, Point::new(0., 0., 1.));
    }

    #[test]
    pub fn rotating_point_around_y() {
        let p = Point::new(0., 0., 1.);
        let res1 = p.rotate_y(PI / 4.);
        let res2 = p.rotate_y(PI / 2.);

        assert_eq!(res1, Point::new(2_f32.sqrt() / 2., 0., 2_f32.sqrt() / 2.));
        assert_eq!(res2, Point::new(1., 0., 0.));
    }

    #[test]
    pub fn rotating_point_around_z() {
        let p = Point::new(0., 1., 0.);
        let res1 = p.rotate_z(PI / 4.);
        let res2 = p.rotate_z(PI / 2.);

        assert_eq!(
            res1,
            Point::new(-(2_f32.sqrt()) / 2., 2_f32.sqrt() / 2., 0.)
        );
        assert_eq!(res2, Point::new(-1., 0., 0.));
    }

    #[test_case((0., 1., 0., 0., 0., 0.), Point::new(6., 3., 4.) ; "moves x in proportion to z")]
    #[test_case((0., 0., 1., 0., 0., 0.), Point::new(2., 5., 4.) ; "moves y in proportion to x")]
    #[test_case((0., 0., 0., 1., 0., 0.), Point::new(2., 7., 4.) ; "moves y in proportion to z")]
    #[test_case((0., 0., 0., 0., 1., 0.), Point::new(2., 3., 6.) ; "moves z in proportion to x")]
    #[test_case((0., 0., 0., 0., 0., 1.), Point::new(2., 3., 7.) ; "moves z in proportion to y")]
    pub fn shearing_point(t: (f32, f32, f32, f32, f32, f32), expected: Point) {
        let p = Point::new(2., 3., 4.).shear(t.0, t.1, t.2, t.3, t.4, t.5);
        assert_eq!(p, expected);
    }

    #[test]
    pub fn composing_transforms() {
        let p = Point::new(1., 0., 1.);
        let A = p.rotate_x(PI / 2.);
        assert_eq!(A, Point::new(1., -1., 0.));
        let B = A.scale(&Vector::new(5., 5., 5.));
        assert_eq!(B, Point::new(5., -5., 0.));
        let C = B.translate(&Vector::new(10., 5., 7.));
        assert_eq!(C, Point::new(15., 0., 7.));
    }

    #[test]
    pub fn composing_transforms_fluent() {
        let p = Point::new(1., 0., 1.);
        let res = p
            .rotate_x(PI / 2.)
            .scale(&Vector::new(5., 5., 5.))
            .translate(&Vector::new(10., 5., 7.));
        assert_eq!(res, Point::new(15., 0., 7.));
    }
}
