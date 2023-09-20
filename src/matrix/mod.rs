use crate::tuple::{Point, Vector};
use lazy_static::lazy_static;
use nalgebra::{matrix, Point4, Vector4};

type Matrix4 = nalgebra::Matrix4<f32>;
type Matrix3 = nalgebra::Matrix3<f32>;
type Matrix2 = nalgebra::Matrix2<f32>;

lazy_static! {
    pub static ref AXIS_X: nalgebra::Unit<nalgebra::Vector3<f32>> =
        nalgebra::Unit::new_normalize(nalgebra::Vector3::new(1., 0., 0.));
    pub static ref AXIS_Y: nalgebra::Unit<nalgebra::Vector3<f32>> =
        nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 1., 0.));
    pub static ref AXIS_Z: nalgebra::Unit<nalgebra::Vector3<f32>> =
        nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 0., 1.));
}

impl Point {
    pub fn translate(&self, translation: &nalgebra::Vector3<f32>) -> Self {
        let p: Point4<f32> = (*self).into();
        let t = Matrix4::new_translation(translation);
        (t * p).into()
    }

    pub fn inverse_translation(&self, translation: &nalgebra::Vector3<f32>) -> Self {
        let p: Point4<f32> = (*self).into();
        let t = Matrix4::new_translation(translation).try_inverse().unwrap();
        (t * p).into()
    }

    pub fn scale(&self, scale: &nalgebra::Vector3<f32>) -> Self {
        let p: Point4<f32> = (*self).into();
        let t = Matrix4::new_nonuniform_scaling(scale);
        (t * p).into()
    }

    pub fn inverse_scale(&self, scale: &nalgebra::Vector3<f32>) -> Self {
        let p: Point4<f32> = (*self).into();
        let t = Matrix4::new_nonuniform_scaling(scale)
            .try_inverse()
            .unwrap();
        (t * p).into()
    }

    fn rotate(&self, axis: &nalgebra::Unit<nalgebra::Vector3<f32>>, angle: f32) -> Self {
        let p: Point4<f32> = (*self).into();
        let t = Matrix4::from_axis_angle(axis, angle);
        (t * p).into()
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        self.rotate(&AXIS_X, angle)
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        self.rotate(&AXIS_Y, angle)
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        self.rotate(&AXIS_Z, angle)
    }

    pub fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let p: Point4<f32> = (*self).into();
        let t = matrix![
            1., xy, xz, 0.;
            yx, 1., yz, 0.;
            zx, zy, 1., 0.;
            0., 0., 0., 1.
        ];
        (t * p).into()
    }
}

impl Vector {
    pub fn scale(&self, scale: &nalgebra::Vector3<f32>) -> Self {
        let p: Vector4<f32> = (*self).into();
        let t = Matrix4::new_nonuniform_scaling(scale);
        (t * p).into()
    }

    pub fn inverse_scale(&self, scale: &nalgebra::Vector3<f32>) -> Self {
        let p: Vector4<f32> = (*self).into();
        let t = Matrix4::new_nonuniform_scaling(scale)
            .try_inverse()
            .unwrap();
        (t * p).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{Matrix2, Matrix3, Matrix4};
    use crate::tuple::{approx_eq, Point, Vector};
    use nalgebra::{matrix, Point4, Vector3};
    use std::f32::consts::PI;
    use test_case::test_case;

    #[test]
    pub fn constructing_matrix4() {
        let m: Matrix4 = matrix![
            1., 2., 3., 4.;
            5.5, 6.5, 7.5, 8.5;
            9., 10., 11., 12.;
            13.5, 14.5, 15.5, 16.5
        ];

        assert_eq!(m[(0, 0)], 1.);
        assert_eq!(m[(0, 3)], 4.);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    pub fn constructing_matrix2() {
        let m: Matrix2 = matrix![
            -3., 5.;
            1., -2.
        ];
        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(0, 1)], 5.);
        assert_eq!(m[(1, 0)], 1.);
        assert_eq!(m[(1, 1)], -2.);
    }

    #[test]
    pub fn constructing_matrix3() {
        let m: Matrix3 = matrix![
            -3., 5., 0.;
            1., -2., -7.;
            0., 1., 1.
        ];
        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(1, 1)], -2.);
        assert_eq!(m[(2, 2)], 1.);
    }

    #[test]
    pub fn comparing_identical_matrix4() {
        let a: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ];
        let b: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ];

        assert_eq!(a, b);
    }

    #[test]
    pub fn comparing_different_matrix4() {
        let a: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ];
        let b: Matrix4 = matrix![
            2., 3., 4., 5.;
            6., 7., 8., 9.;
            8., 7., 6., 5.;
            4., 3., 2., 1.
        ];

        assert_ne!(a, b);
    }

    #[test]
    pub fn multiplying_matrices() {
        let a: Matrix4 = matrix![
            1., 2., 3., 4.;
            5., 6., 7., 8.;
            9., 8., 7., 6.;
            5., 4., 3., 2.
        ];
        let b: Matrix4 = matrix![
            -2., 1., 2., 3.;
            3., 2., 1., -1.;
            4., 3., 6., 5.;
            1., 2., 7., 8.
        ];

        let res: Matrix4 = matrix![
            20., 22., 50., 48.;
            44., 54., 114., 108.;
            40., 58., 110., 102.;
            16., 26., 46., 42.
        ];

        assert_eq!(a * b, res);
    }

    #[test]
    pub fn multiply_by_point() {
        let A: Matrix4 = matrix![
            1., 2., 3., 4.;
            2., 4., 4., 2.;
            8., 6., 4., 1.;
            0., 0., 0., 1.
        ];
        let b: nalgebra::Point4<f32> = Point::new(1., 2., 3.).into();
        let res: Point = (A * b).into();

        assert_eq!(res, Point::new(18., 24., 33.));
    }

    #[test]
    pub fn multiplying_matrix_by_identity_matrix() {
        let a: Matrix4 = matrix![
            0., 1., 2., 4.;
            1., 2., 4., 8.;
            2., 4., 8., 16.;
            4., 8., 16., 32.
        ];
        let b = Matrix4::identity();
        assert_eq!(a * b, a);
    }

    #[test]
    pub fn multiplying_point_by_identity_matrix() {
        let a: Point4<f32> = Point::new(1., 2., 3.).into();
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
        ];
        let transposed: Matrix4 = matrix![
            0., 9., 1., 0.;
            9., 8., 8., 0.;
            3., 0., 5., 5.;
            0., 8., 3., 8.
        ];

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
        ];
        let b: Matrix4 = matrix![
            8., 2., 2., 2.;
            3., -1., 7., 0.;
            7., 0., 5., 4.;
            6., -2., 0., 5.
        ];
        let c = a * b;
        let res = c * b.try_inverse().unwrap();

        assert!(res
            .data
            .as_slice()
            .iter()
            .zip(a.data.as_slice().iter())
            .all(|(&a, &b)| approx_eq(a, b)));
    }

    #[test]
    pub fn translate_point() {
        let p = Point::new(5., -3., 2.);
        let res = p.translate(&nalgebra::Vector3::new(-3., 4., 5.));
        assert_eq!(res, Point::new(2., 1., 7.));
    }

    #[test]
    pub fn inverse_undoes_translation() {
        let p = Point::new(-3., 4., 5.);
        let res = p.inverse_translation(&nalgebra::Vector3::new(5., -3., 2.));
        assert_eq!(res, Point::new(-8., 7., 3.));
    }

    #[test]
    pub fn scaling_point() {
        let p = Point::new(-4., 6., 8.);
        let res = p.scale(&nalgebra::Vector3::new(2., 3., 4.));
        assert_eq!(res, Point::new(-8., 18., 32.));
    }

    #[test]
    pub fn inverse_undoes_scale_point() {
        let p = Point::new(-4., 6., 8.);
        let res = p.inverse_scale(&nalgebra::Vector3::new(2., 3., 4.));
        assert_eq!(res, Point::new(-2., 2., 2.));
    }

    #[test]
    pub fn scaling_vector() {
        let p = Vector::new(-4., 6., 8.);
        let res = p.scale(&nalgebra::Vector3::new(2., 3., 4.));
        assert_eq!(res, Vector::new(-8., 18., 32.));
    }

    #[test]
    pub fn inverse_undoes_scale_vector() {
        let p = Vector::new(-4., 6., 8.);
        let res = p.inverse_scale(&nalgebra::Vector3::new(2., 3., 4.));
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
        let B = A.scale(&Vector3::new(5., 5., 5.));
        assert_eq!(B, Point::new(5., -5., 0.));
        let C = B.translate(&Vector3::new(10., 5., 7.));
        assert_eq!(C, Point::new(15., 0., 7.));
    }

    #[test]
    pub fn composing_transforms_fluent() {
        let p = Point::new(1., 0., 1.);
        let res = p
            .rotate_x(PI / 2.)
            .scale(&Vector3::new(5., 5., 5.))
            .translate(&Vector3::new(10., 5., 7.));
        assert_eq!(res, Point::new(15., 0., 7.));
    }
}
