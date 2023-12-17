use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::{Intersection, Shape};
use crate::tuple::{approx_cmp, approx_eq, Point, Vector, EPSILON};
use smallvec::{smallvec, SmallVec};
use std::cmp::Ordering;
use uuid::Uuid;

pub struct Cylinder {
    pub id: Uuid,
    pub transform: Matrix4,
    pub inverse_transform: Matrix4,
    pub material: Material,
    pub minimum: f32,
    pub maximum: f32,
    pub is_closed: bool,
}

unsafe impl Send for Cylinder {}
unsafe impl Sync for Cylinder {}

impl Shape for Cylinder {
    fn local_intersect(&'static self, ray: &Ray) -> Option<SmallVec<[Intersection; 8]>> {
        let a = ray.direction.z.mul_add(ray.direction.z, ray.direction.x.powi(2));
        if approx_eq(a, 0.) {
            return if self.is_closed {
                let mut res = smallvec![];
                self.intersect_caps(ray, &mut res);
                Some(res)
            } else {
                None
            };
        }

        let b = (2. * ray.origin.x).mul_add(ray.direction.x, 2. * ray.origin.z * ray.direction.z);
        let c = ray.origin.z.mul_add(ray.origin.z, ray.origin.x.powi(2)) - 1.;

        let discriminant = b.mul_add(b, -(4. * a * c));
        if approx_cmp(discriminant, 0.) == Ordering::Less {
            return None;
        }

        let mut t0 = (-b - discriminant.sqrt()) / (2. * a);
        let mut t1 = (-b + discriminant.sqrt()) / (2. * a);

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        let mut res = smallvec![];

        let y0 = t0.mul_add(ray.direction.y, ray.origin.y);
        if approx_cmp(self.minimum, y0) == Ordering::Less
            && approx_cmp(y0, self.maximum) == Ordering::Less
        {
            res.push(Intersection::new(t0, self));
        }

        let y1 = t1.mul_add(ray.direction.y, ray.origin.y);
        if approx_cmp(self.minimum, y1) == Ordering::Less
            && approx_cmp(y1, self.maximum) == Ordering::Less
        {
            res.push(Intersection::new(t1, self));
        }

        self.intersect_caps(ray, &mut res);
        Some(res)
    }

    fn local_normal(&self, p: &Point) -> Vector {
        let distance = p.z.mul_add(p.z, p.x.powi(2));

        if approx_cmp(distance, 1.) == Ordering::Less && p.y >= self.maximum - EPSILON {
            return Vector::new(0., 1., 0.);
        }

        if approx_cmp(distance, 1.) == Ordering::Less && p.y <= self.minimum + EPSILON {
            return Vector::new(0., -1., 0.);
        }

        Vector::new(p.x, 0., p.z)
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

impl Cylinder {
    fn check_cap(ray: &Ray, t: f32) -> bool {
        let x = t.mul_add(ray.direction.x, ray.origin.x);
        let z = t.mul_add(ray.direction.z, ray.origin.z);
        let comp = approx_cmp(z.mul_add(z, x.powi(2)), 1.);
        comp == Ordering::Less || comp == Ordering::Equal
    }

    fn intersect_caps(&'static self, ray: &Ray, xs: &mut SmallVec<[Intersection; 8]>) {
        if !self.is_closed || approx_eq(ray.direction.y, 0.) {
            return;
        }

        let t0 = (self.minimum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t0) {
            xs.push(Intersection::new(t0, self));
        }

        let t1 = (self.maximum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t1) {
            xs.push(Intersection::new(t1, self));
        }
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity().inverse(),
            material: Material::default(),
            minimum: f32::NEG_INFINITY,
            maximum: f32::INFINITY,
            is_closed: false,
        }
    }
}

impl Cylinder {
    pub fn static_default() -> &'static mut Self {
        Box::leak(Box::default())
    }
    pub fn default_with_material(material: Material) -> &'static mut Self {
        let c = Self::static_default();
        c.material = material;
        c
    }

    pub fn set_transform(&'static mut self, transform: Matrix4) -> &'static mut Self {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::shape::{Cylinder, Shape};
    use crate::tuple::{Point, Vector};
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(Point::new(1., 0., 0.), Vector::new(0., 1., 0.))]
    #[test_case(Point::new(0., 0., 0.), Vector::new(0., 1., 0.))]
    #[test_case(Point::new(0., 0., -5.), Vector::new(1., 1., 1.))]
    fn ray_misses_a_cylinder(p: Point, v: Vector) {
        let c = Cylinder::static_default();
        let normalized_direction = v.normalize();
        let r = Ray::new(p, normalized_direction);
        let xs = c.local_intersect(&r);
        assert!(xs.is_none());
    }

    #[test_case(Point::new(1., 0., -5.), Vector::new(0., 0., 1.), 5., 5.)]
    #[test_case(Point::new(0., 0., -5.), Vector::new(0., 0., 1.), 4., 6.)]
    #[test_case(Point::new(0.5, 0., -5.), Vector::new(0.1, 1., 1.), 6.808_006, 7.088_698_4)]
    fn ray_strikes_cylinder(p: Point, v: Vector, t0: f32, t1: f32) {
        let c = Cylinder::static_default();
        let normalized_direction = v.normalize();
        let r = Ray::new(p, normalized_direction);
        let xs = c.local_intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, t0);
        assert_eq!(xs[1].t, t1);
    }

    #[test_case(Point::new(1., 0., 0.), Vector::new(1., 0., 0.))]
    #[test_case(Point::new(0., 5., -1.), Vector::new(0., 0., -1.))]
    #[test_case(Point::new(0., -2., 1.), Vector::new(0., 0., 1.))]
    #[test_case(Point::new(-1., 1., 0.), Vector::new(-1., 0., 0.))]
    fn normal_on_a_cylinder(p: Point, n: Vector) {
        let c = Cylinder::static_default();
        let normal = c.local_normal(&p);
        assert_eq!(normal, n);
    }

    #[test_case(Point::new(0., 1.5, 0.), Vector::new(0.1, 1., 0.), 0)]
    #[test_case(Point::new(0., 3., -5.), Vector::new(0., 0., -1.), 0)]
    #[test_case(Point::new(0., 0., -5.), Vector::new(0., 0., -1.), 0)]
    #[test_case(Point::new(0., 2., -5.), Vector::new(0., 0., -1.), 0)]
    #[test_case(Point::new(0., 1., -5.), Vector::new(0., 0., -1.), 0)]
    #[test_case(Point::new(0., 1.5, -2.), Vector::new(0., 0., 1.), 2)]
    fn intersecting_a_constrained_cylinder(p: Point, v: Vector, count: usize) {
        let c = Cylinder::default_with_material(Default::default());
        c.minimum = 1.;
        c.maximum = 2.;

        let normalized_direction = v.normalize();
        let r = Ray::new(p, normalized_direction);
        let xs = c.local_intersect(&r);
        assert!(xs.is_some());
        assert_eq!(xs.unwrap().len(), count);
    }

    #[test_case(Point::new(0., 3., 0.), Vector::new(0., -1., 0.), 2)]
    #[test_case(Point::new(0., 3., -2.), Vector::new(0., -1., 2.), 2)]
    #[test_case(Point::new(0., 4., -2.), Vector::new(0., -1., 1.), 2)]
    #[test_case(Point::new(0., 0., -2.), Vector::new(0., 1., 2.), 2)]
    #[test_case(Point::new(0., -1., -2.), Vector::new(0., 1., 1.), 2)]
    fn intersecting_the_caps_of_a_closed_cylinder(p: Point, v: Vector, count: usize) {
        let c = Cylinder::default_with_material(Default::default());
        c.minimum = 1.;
        c.maximum = 2.;
        c.is_closed = true;

        let normalized_direction = v.normalize();
        let r = Ray::new(p, normalized_direction);
        let xs = c.local_intersect(&r).unwrap();
        assert_eq!(xs.len(), count);
    }

    #[test_case(Point::new(0., 1., 0.), Vector::new(0., -1., 0.))]
    #[test_case(Point::new(0.5, 1., 0.), Vector::new(0., -1., 0.))]
    #[test_case(Point::new(0., 1., 0.5), Vector::new(0., -1., 0.))]
    #[test_case(Point::new(0., 2., 0.), Vector::new(0., 1., 0.))]
    #[test_case(Point::new(0.5, 2., 0.), Vector::new(0., 1., 0.))]
    #[test_case(Point::new(0., 2., 0.5), Vector::new(0., 1., 0.))]
    fn normal_vector_at_cylinder_end_caps(p: Point, n: Vector) {
        let c = Cylinder::default_with_material(Default::default());
        c.minimum = 1.;
        c.maximum = 2.;
        c.is_closed = true;

        let normal = c.local_normal(&p);
        assert_eq!(normal, n);
    }
}
