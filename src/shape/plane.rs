use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::{Intersection, Shape};
use crate::tuple::{Point, Vector, EPSILON};
use derive_more::Constructor;
use smallvec::{smallvec, SmallVec};
use uuid::Uuid;

#[derive(Debug, Constructor)]
pub struct Plane {
    id: Uuid,
    transform: Matrix4,
    inverse_transform: Matrix4,
    material: Material,
}

impl Plane {
    pub fn static_default() -> &'static mut Self {
        Box::leak(Box::default())
    }

    pub fn default_with_material(m: Material) -> &'static mut Self {
        Box::leak(Box::new(Self {
            material: m,
            ..Default::default()
        }))
    }

    pub fn set_transform(&'static mut self, transform: Matrix4) -> &'static mut Self {
        self.transform = transform;
        self.inverse_transform = self.transform.inverse();
        self
    }
}

unsafe impl Send for Plane {}
unsafe impl Sync for Plane {}

impl Default for Plane {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity().inverse(),
            material: Material::default(),
        }
    }
}

impl Shape for Plane {
    fn local_intersect(&'static self, ray: &Ray) -> Option<SmallVec<[Intersection; 8]>> {
        if ray.direction.y.abs() < EPSILON {
            return None;
        }
        let t = -ray.origin.y / ray.direction.y;
        Some(smallvec![Intersection::new(t, self)])
    }

    fn local_normal(&self, _p: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
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

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::shape::plane::Plane;
    use crate::shape::Shape;
    use crate::tuple::{Point, Vector};
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(Point::new(0., 0., 0.))]
    #[test_case(Point::new(10., 0., -10.))]
    #[test_case(Point::new(-5., 0., 150.))]
    pub fn normal_of_plane_is_constant_everywhere(p: Point) {
        let plane = Plane::default();
        assert_eq!(plane.local_normal(&p), Vector::new(0., 1., 0.));
    }

    #[test_case(Ray::new(Point::new(0., 10., 0.), Vector::new(0., 0., 1.)), None ; "intersect with parallel ray")]
    #[test_case(Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.)), None ; "intersect with coplanar ray")]
    #[test_case(Ray::new(Point::new(0., 1., 0.), Vector::new(0., -1., 0.)), Some(1.) ; "intersect with ray from above")]
    #[test_case(Ray::new(Point::new(0., -1., 0.), Vector::new(0., 1., 0.)), Some(1.) ; "intersect with ray from below")]
    pub fn intersect_ray_with_parallel_plane(r: Ray, expected: Option<f32>) {
        let plane = Plane::static_default();
        let xs = plane.local_intersect(&r);
        assert_eq!(xs.map(|xs| xs[0].t), expected);
    }
}
