use std::mem::swap;

use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::{Intersection, Shape};
use crate::tuple::{approx_cmp, Point, Vector, EPSILON};
use smallvec::{smallvec, SmallVec};
use uuid::Uuid;

pub struct Cube {
    transform: Matrix4,
    inverse_transform: Matrix4,
    id: Uuid,
    material: Material,
}

impl Cube {
    pub fn static_default() -> &'static mut Self {
        Box::leak(Box::default())
    }

    pub fn default_with_material(m: Material) -> &'static mut Self {
        let c = Self::static_default();
        c.material = m;
        c
    }

    pub fn set_transform(&mut self, t: Matrix4) {
        self.transform = t;
        self.inverse_transform = self.transform.inverse();
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity().inverse(),
            id: Uuid::new_v4(),
            material: Material::default(),
        }
    }
}

unsafe impl Send for Cube {}
unsafe impl Sync for Cube {}

impl Shape for Cube {
    fn local_intersect(&'static self, ray: &Ray) -> Option<SmallVec<[Intersection; 8]>> {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = [xtmin, ytmin, ztmin]
            .into_iter()
            .max_by(|&a, &b| approx_cmp(a, b))
            .unwrap();
        let tmax = [xtmax, ytmax, ztmax]
            .into_iter()
            .min_by(|&a, &b| approx_cmp(a, b))
            .unwrap();

        if tmin > tmax {
            return None;
        }

        Some(smallvec![
            Intersection::new(tmin, self),
            Intersection::new(tmax, self)
        ])
    }

    fn local_normal(&self, p: &Point) -> Vector {
        let maxc = [p.x.abs(), p.y.abs(), p.z.abs()]
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if maxc == p.x.abs() {
            Vector::new(p.x, 0., 0.)
        } else if maxc == p.y.abs() {
            Vector::new(0., p.y, 0.)
        } else {
            Vector::new(0., 0., p.z)
        }
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn get_inverse_transform(&self) -> &Matrix4 {
        &self.inverse_transform
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let tmin_numerator = -1. - origin;
    let tmax_numerator = 1. - origin;
    let mut tmin;
    let mut tmax;

    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * f32::INFINITY;
        tmax = tmax_numerator * f32::INFINITY;
    }

    if tmin > tmax {
        swap(&mut tmin, &mut tmax);
    }

    (tmin, tmax)
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::shape::{Cube, Shape};
    use crate::tuple::{Point, Vector};
    use test_case::test_case;

    #[test_case(Point::new(5., 0.5, 0.), Vector::new(-1., 0., 0.), 4., 6. ; "positive x")]
    #[test_case(Point::new(-5., 0.5, 0.), Vector::new(1., 0., 0.), 4., 6. ; "negative x")]
    #[test_case(Point::new(0.5, 5.0, 0.), Vector::new(0., -1., 0.), 4., 6. ; "positive y")]
    #[test_case(Point::new(0.5, -5.0, 0.), Vector::new(0., 1., 0.), 4., 6. ; "negative y")]
    #[test_case(Point::new(0.5, 0.0, 5.), Vector::new(0., 0., -1.), 4., 6. ; "positive z")]
    #[test_case(Point::new(0.5, 0.0, -5.), Vector::new(0., 0., 1.), 4., 6. ; "negative z")]
    #[test_case(Point::new(0., 0.5, 0.), Vector::new(0., 0., 1.), -1., 1. ; "inside")]
    pub fn ray_intersects_cube(origin: Point, direction: Vector, t1: f32, t2: f32) {
        let c = Cube::static_default();
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, t1);
        assert_eq!(xs[1].t, t2);
    }

    #[test_case(Point::new(-2., 0.0, 0.), Vector::new(0.2673, 0.5345, 0.8018))]
    #[test_case(Point::new(0., -2.0, 0.), Vector::new(0.8018, 0.2673, 0.5345))]
    #[test_case(Point::new(0., 0., -2.), Vector::new(0.5345, 0.8018, 0.2673))]
    #[test_case(Point::new(2., 0., 2.), Vector::new(0., 0., -1.))]
    #[test_case(Point::new(0., 2., 2.), Vector::new(0., -1., 0.))]
    #[test_case(Point::new(2., 2., 0.), Vector::new(-1., 0., 0.))]
    pub fn ray_misses_cube(origin: Point, direction: Vector) {
        let c = Cube::static_default();
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r);
        assert!(xs.is_none());
    }

    #[test_case(Point::new(1., -0.5, -0.8), Vector::new(1., 0., 0.))]
    #[test_case(Point::new(-1., -0.2, 0.9), Vector::new(-1., 0., 0.))]
    #[test_case(Point::new(-0.4, 1.0, -0.1), Vector::new(0., 1., 0.))]
    #[test_case(Point::new(0.3, -1.0, -0.7), Vector::new(0., -1., 0.))]
    #[test_case(Point::new(-0.6, 0.3, 1.0), Vector::new(0., 0., 1.))]
    #[test_case(Point::new(0.4, 0.4, -1.0), Vector::new(0., 0., -1.))]
    #[test_case(Point::new(1.0, 1.0, 1.0), Vector::new(1., 0., 0.) ; "corner one") ]
    #[test_case(Point::new(-1.0, -1.0, -1.0), Vector::new(-1., 0., 0.); "corner two") ]
    pub fn normal_at_surface_of_cube(point: Point, expected: Vector) {
        let c = Cube::static_default();
        let normal = c.local_normal(&point);
        assert_eq!(normal, expected);
    }
}
