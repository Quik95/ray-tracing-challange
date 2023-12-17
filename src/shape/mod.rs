mod cube;
mod plane;
mod sphere;

pub use cube::Cube;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::ray::Ray;
use derive_more::Constructor;
use itertools::Itertools;
use smallvec::SmallVec;
use std::cmp::Ordering;
use uuid::Uuid;

use crate::material::Material;
use crate::matrix::Matrix4;
use crate::tuple::{Point, Vector, EPSILON};

pub trait Shape: Send + Sync {
    fn local_intersect(&'static self, ray: &Ray) -> Option<SmallVec<[Intersection; 8]>>;
    fn intersect(&'static self, ray: &Ray) -> Option<SmallVec<[Intersection; 8]>> {
        let ray = ray.transform(self.get_inverse_transform());
        self.local_intersect(&ray)
    }
    fn local_normal(&self, p: &Point) -> Vector;
    fn get_normal(&self, point: &Point) -> Vector {
        let local_point = self.get_inverse_transform() * point;
        let local_normal = self.local_normal(&local_point);
        let world_normal = self.get_inverse_transform().transpose() * local_normal;

        world_normal.normalize()
    }
    fn get_material(&self) -> &Material;
    fn get_transform(&self) -> &Matrix4;
    fn get_inverse_transform(&self) -> &Matrix4;
    fn get_id(&self) -> &Uuid;
}

impl Eq for dyn Shape {}
impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

#[derive(Constructor, Copy, Clone)]
pub struct Intersection {
    pub t: f32,
    pub object: &'static dyn Shape,
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
    pub fn get_hit(hits: &[Self]) -> Option<Self> {
        hits.iter()
            .filter(|&&x| x.t >= 0.)
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
            .copied()
    }

    fn calculate_refractive_indices(&self, xs: &[Self]) -> (f32, f32) {
        let mut n1 = 0.0;
        let mut n2 = 0.0;

        let mut containers: Vec<&'_ dyn Shape> = vec![];
        for i in xs {
            if i == self {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().get_material().refractive_index;
                }
            }

            if let Some((index, _)) = containers.iter().find_position(|&x| x == &i.object) {
                containers.remove(index);
            } else {
                containers.push(i.object);
            }

            if i == self {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().get_material().refractive_index;
                }

                break;
            }
        }

        (n1, n2)
    }

    pub fn precompute_hit(self, ray: &Ray, xs: &[Self]) -> PrecomputedHit {
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
        let over_point = point + normal * EPSILON;
        let under_point = point - normal * EPSILON;
        let reflected = ray.direction.reflect(&normal);
        let (n1, n2) = self.calculate_refractive_indices(xs);

        PrecomputedHit {
            intersection: self,
            point,
            eye,
            normal,
            inside,
            over_point,
            under_point,
            reflected_vector: reflected,
            n1,
            n2,
        }
    }
}

#[derive(Constructor, Copy, Clone)]
pub struct PrecomputedHit {
    pub intersection: Intersection,
    pub point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub under_point: Point,
    pub reflected_vector: Vector,
    pub n1: f32,
    pub n2: f32,
}

impl PrecomputedHit {
    pub fn schlick_reflectance(&self) -> f32 {
        let mut cos = self.eye.dot(&self.normal);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2t = n * n * cos.mul_add(-cos, 1.0);
            if sin2t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::shape::{Intersection, Plane, Sphere};
    use crate::tuple::{Point, Vector, EPSILON};

    use pretty_assertions::assert_eq;

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
        let ph = i.precompute_hit(&r, &[i]);
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
        let ph = i.precompute_hit(&r, &[i]);
        assert_eq!(ph.point, Point::new(0., 0., 1.));
        assert_eq!(ph.eye, Vector::new(0., 0., -1.));
        assert_eq!(ph.normal, Vector::new(0., 0., -1.));
        assert!(ph.inside);
    }

    #[test]
    pub fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let shape = Sphere::static_default()
            .set_transform(Matrix4::identity().translate(Vector::new(0., 0., 1.)));
        let i = Intersection::new(5., shape);
        let comps = i.precompute_hit(&r, &[i]);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    pub fn hit_refractive_should_offset_point() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_glass_sphere()
            .set_transform(Matrix4::identity().translate(Vector::new(0., 0., 1.)));
        let i = Intersection::new(5., s);
        let comps = i.precompute_hit(&r, &[i]);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    pub fn precomputing_reflection_vector() {
        let r = Ray::new(
            Point::new(0., 1., -1.),
            Vector::new(
                0.,
                -std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2,
            ),
        );
        let i = Intersection::new(2.0_f32.sqrt(), Plane::static_default());
        let comps = i.precompute_hit(&r, &[i]);
        assert_eq!(
            comps.reflected_vector,
            Vector::new(
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2
            )
        );
    }

    #[test]
    pub fn schlick_under_total_internal_reflection() {
        let s = Sphere::static_glass_sphere();
        let r = Ray::new(
            Point::new(0., 0., std::f32::consts::FRAC_1_SQRT_2),
            Vector::new(0., 1., 0.),
        );
        let i = vec![
            Intersection::new(-std::f32::consts::FRAC_1_SQRT_2, s),
            Intersection::new(std::f32::consts::FRAC_1_SQRT_2, s),
        ];
        let comps = i[1].precompute_hit(&r, &i);
        let reflectance = comps.schlick_reflectance();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    pub fn schlick_with_perpendicular_angle() {
        let s = Sphere::static_glass_sphere();
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 1., 0.));
        let i = vec![Intersection::new(-1., s), Intersection::new(1., s)];
        let comps = i[1].precompute_hit(&r, &i);
        let reflectance = comps.schlick_reflectance();
        assert_eq!(reflectance, 0.040_000_003);
    }

    #[test]
    pub fn schlick_reflactance_with_small_angle_and_n2_gt_n1() {
        let s = Sphere::static_glass_sphere();
        let r = Ray::new(Point::new(0., 0.99, -2.), Vector::new(0., 0., 1.));
        let i = vec![Intersection::new(1.8589, s)];
        let comps = i[0].precompute_hit(&r, &i);
        let reflectance = comps.schlick_reflectance();
        assert_eq!(reflectance, 0.488_730_67);
    }
}
