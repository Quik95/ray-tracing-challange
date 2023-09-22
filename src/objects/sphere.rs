use crate::matrix::Matrix4;
use crate::objects::{Hittable, Intersection};
use crate::ray::Ray;
use crate::tuple::Point;

use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: nalgebra::Matrix4<f32>,
}

impl Sphere {
    pub fn static_default() -> &'static mut Self {
        let s = Box::<Sphere>::default();
        let leaked = Box::leak(s);
        leaked
    }

    pub fn transform(&'static mut self, &transform: &Matrix4) -> &'static mut Self {
        self.transform = transform * self.transform;
        self
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix4::identity(),
        }
    }
}

impl Hittable for Sphere {
    fn intersect(&'static self, ray: &Ray) -> Option<Intersection> {
        let origin = Point::zero();
        let radius = 1.0;
        let ray = ray.transform(&self.transform.try_inverse().unwrap());

        let sphere_to_ray = ray.origin - origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - radius * radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        Some(Intersection::new(vec![t1, t2], self))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::objects::{Hittable, Sphere};
    use crate::ray::Ray;
    use crate::tuple::{Point, Vector};
    use nalgebra::vector;

    #[test]
    pub fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., -5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots.roots[0], 4.);
        assert_eq!(roots.roots[1], 6.);
    }

    #[test]
    pub fn ray_intersects_sphere_at_one_point() {
        let r = Ray::new(
            crate::tuple::Point::new(0., 1., -5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots.roots[0], 5.);
        assert_eq!(roots.roots[1], 5.);
    }

    #[test]
    pub fn ray_missed_sphere() {
        let r = Ray::new(
            crate::tuple::Point::new(0., 2., -5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let s = Sphere::static_default();
        let roots = s.intersect(&r);
        assert!(roots.is_none());
    }

    #[test]
    pub fn ray_originates_inside_sphere() {
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., 0.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots.roots[0], -1.);
        assert_eq!(roots.roots[1], 1.);
    }

    #[test]
    pub fn ray_is_behind_sphere() {
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., 5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots.roots[0], -6.);
        assert_eq!(roots.roots[1], -4.);
    }

    #[test]
    pub fn changing_the_sphere_transform() {
        let s = Sphere::static_default();
        let t = Matrix4::new_translation(&vector![2., 3., 4.]);
        let s2 = s.transform(&t);
        assert_eq!(s2.transform, t);
    }

    #[test]
    pub fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default()
            .transform(&Matrix4::new_nonuniform_scaling(&vector![2., 2., 2.]));
        let intersects = s.intersect(&r).unwrap();
        assert_eq!(intersects.roots[0], 3.);
        assert_eq!(intersects.roots[1], 7.);
    }

    pub fn intersect_translated_ray_with_sphere() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default().transform(&Matrix4::new_translation(&vector![5., 0., 0.]));
        let intersects = s.intersect(&r);
        assert!(intersects.is_none());
    }
}
