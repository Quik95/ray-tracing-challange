mod sphere;

use crate::ray::Ray;
use derive_more::Constructor;
use std::cmp::Ordering;
use uuid::Uuid;

use crate::material::Material;
use crate::tuple::{Point, Vector};
pub use sphere::Sphere;

pub trait Hittable {
    fn intersect(&'static self, ray: &Ray) -> Option<Vec<Intersection>>;
    fn get_normal(&self, point: &Point) -> Vector;
    fn get_material(&self) -> &Material;
    fn get_id(&self) -> &Uuid;
}

#[derive(Constructor, Copy, Clone)]
pub struct Intersection {
    pub t: f32,
    pub object: &'static dyn Hittable,
}

impl Eq for Intersection {}

impl PartialEq<Self> for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object.get_id() == other.object.get_id()
    }
}

impl PartialOrd<Self> for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Intersection {
    pub fn get_hit(hits: &[Self]) -> Option<Intersection> {
        hits.iter()
            .filter(|&&x| x.t >= 0.)
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
            .copied()
    }

    pub fn precompute_hit(self, ray: &Ray) -> PrecomputedHit {
        let point = ray.position(self.t);
        let eye = -ray.direction;
        let mut normal = self.object.get_normal(&point);
        let inside;

        if normal.dot(&eye) < 0. {
            normal = -normal;
            inside = true;
        } else {
            inside = false;
        }

        PrecomputedHit::new(self, point, eye, normal, inside)
    }
}

#[derive(Constructor, Copy, Clone)]
pub struct PrecomputedHit {
    pub intersection: Intersection,
    pub point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub inside: bool,
}

#[cfg(test)]
mod tests {
    use crate::objects::{Intersection, Sphere};
    use crate::ray::Ray;
    use crate::tuple::{Point, Vector};

    #[test]
    pub fn when_all_t_positive() {
        let s = Sphere::static_default();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(2., s);
        let h = Intersection::get_hit(&[i1, i2]);
        assert_eq!(h.unwrap().t, 1.);
    }

    #[test]
    pub fn when_some_negative_t() {
        let s = Sphere::static_default();
        let i1 = Intersection::new(1., s);
        let i2 = Intersection::new(-1., s);
        let h = Intersection::get_hit(&[i1, i2]);
        assert_eq!(h.unwrap().t, 1.);
    }

    #[test]
    pub fn when_all_negative_t() {
        let s = Sphere::static_default();
        let i1 = Intersection::new(-2., s);
        let i2 = Intersection::new(-1., s);
        let h = Intersection::get_hit(&[i1, i2]);
        assert!(h.is_none());
    }

    #[test]
    pub fn always_lowest_nonnegative() {
        let s = Sphere::static_default();
        let i1 = Intersection::new(5., s);
        let i2 = Intersection::new(-7., s);
        let i3 = Intersection::new(-3., s);
        let i4 = Intersection::new(2., s);
        let h = Intersection::get_hit(&[i1, i2, i3, i4]);
        assert_eq!(h.unwrap().t, 2.);
    }

    #[test]
    pub fn precompute_the_state_of_intersection() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let shape = Sphere::static_default();
        let i = Intersection::new(4., shape);
        let ph = i.precompute_hit(&r);
        assert_eq!(ph.point, Point::new(0., 0., -1.));
        assert_eq!(ph.eye, Vector::new(0., 0., -1.));
        assert_eq!(ph.normal, Vector::new(0., 0., -1.));
        assert!(!ph.inside);
    }

    #[test]
    pub fn hit_when_intersection_inside() {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let shape = Sphere::static_default();
        let i = Intersection::new(1., shape);
        let ph = i.precompute_hit(&r);
        assert_eq!(ph.point, Point::new(0., 0., 1.));
        assert_eq!(ph.eye, Vector::new(0., 0., -1.));
        assert_eq!(ph.normal, Vector::new(0., 0., -1.));
        assert!(ph.inside);
    }
}
