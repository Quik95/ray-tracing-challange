mod sphere;

use crate::ray::Ray;
use derive_more::Constructor;

use crate::material::Material;
use crate::tuple::{Point, Vector};
pub use sphere::Sphere;

pub trait Hittable {
    fn intersect(&'static self, ray: &Ray) -> Option<Intersection>;
    fn get_normal(&self, point: &Point) -> Vector;
    fn get_material(&self) -> &Material;
}

#[derive(Constructor)]
pub struct Intersection {
    pub roots: Vec<f32>,
    pub object: &'static dyn Hittable,
}

impl Intersection {
    pub fn get_hit(&self) -> Option<f32> {
        if self.roots.iter().all(|&x| x < 0.) {
            return None;
        }

        self.roots
            .iter()
            .filter(|&&x| x >= 0.)
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::{Intersection, Sphere};

    #[test]
    pub fn when_all_t_positive() {
        let s = Sphere::static_default();
        let i = Intersection::new(vec![1., 2.], s);
        let h = i.get_hit();
        assert_eq!(h, Some(1.));
    }

    #[test]
    pub fn when_some_negative_t() {
        let s = Sphere::static_default();
        let i = Intersection::new(vec![1., -1.], s);
        let h = i.get_hit();
        assert_eq!(h, Some(1.));
    }

    #[test]
    pub fn when_all_negative_t() {
        let s = Sphere::static_default();
        let i = Intersection::new(vec![-2., -1.], s);
        let h = i.get_hit();
        assert_eq!(h, None);
    }

    #[test]
    pub fn always_lowest_nonnegative() {
        let s = Sphere::static_default();
        let i = Intersection::new(vec![5., 7., -3., 2.], s);
        let h = i.get_hit();
        assert_eq!(h, Some(2.));
    }
}
