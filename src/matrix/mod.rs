type Matrix4 = nalgebra::Matrix4<f32>;
type Matrix3 = nalgebra::Matrix3<f32>;
type Matrix2 = nalgebra::Matrix2<f32>;

#[cfg(test)]
mod tests {
    use crate::matrix::{Matrix2, Matrix3, Matrix4};
    use crate::tuple::{approx_eq, Point};
    use nalgebra::{matrix, Point4};

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
}
