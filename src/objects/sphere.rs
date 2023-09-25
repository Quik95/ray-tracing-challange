use crate::material::Material;
use uuid::Uuid;

use crate::matrix::Matrix4;
use crate::objects::{Intersection, Shape};
use crate::ray::Ray;
use crate::tuple::{Point, Vector};

#[derive(Debug)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrix4,
    pub material: Material,
}

impl Eq for Sphere {}
impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Sphere {
    pub fn static_default() -> &'static mut Self {
        let s = Box::<Self>::default();
        let leaked = Box::leak(s);
        leaked
    }

    pub fn default_with_material(material: Material) -> &'static mut Self {
        let mut s = Box::<Self>::default();
        s.material = material;

        let leaked = Box::leak(s);
        leaked
    }

    pub fn set_transform(&'static mut self, transform: &Matrix4) -> &'static mut Self {
        self.transform = *transform * self.transform;
        self
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl Shape for Sphere {
    fn local_intersect(&'static self, ray: &Ray) -> Option<Vec<Intersection>> {
        let origin = Point::zero();
        let radius = 1.0_f32;
        let sphere_to_ray = ray.origin - origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = radius.mul_add(-radius, sphere_to_ray.dot(&sphere_to_ray));
        let discriminant = b.mul_add(b, -4. * a * c);

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        Some(vec![
            Intersection::new(t1, self),
            Intersection::new(t2, self),
        ])
    }

    fn local_normal(&self, p: &Point) -> Vector {
        (p - Point::zero()).normalize()
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::matrix::Matrix4;
    use crate::objects::{Shape, Sphere};
    use crate::ray::Ray;
    use crate::tuple::{Point, Vector};

    #[test]
    pub fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots[0].t, 4.);
        assert_eq!(roots[1].t, 6.);
    }

    #[test]
    pub fn ray_intersects_sphere_at_one_point() {
        let r = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots[0].t, 5.);
        assert_eq!(roots[1].t, 5.);
    }

    #[test]
    pub fn ray_missed_sphere() {
        let r = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default();
        let roots = s.intersect(&r);
        assert!(roots.is_none());
    }

    #[test]
    pub fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots[0].t, -1.);
        assert_eq!(roots[1].t, 1.);
    }

    #[test]
    pub fn ray_is_behind_sphere() {
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default();
        let roots = s.intersect(&r).unwrap();
        assert_eq!(roots[0].t, -6.);
        assert_eq!(roots[1].t, -4.);
    }

    #[test]
    pub fn changing_the_sphere_transform() {
        let s = Sphere::static_default();
        let t = Matrix4::identity().translate(&Vector::new(2., 3., 4.));
        let s2 = s.set_transform(&t);
        assert_eq!(s2.transform, t);
    }

    #[test]
    pub fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default()
            .set_transform(&Matrix4::identity().scale(&Vector::new(2., 2., 2.)));
        let intersects = s.intersect(&r).unwrap();
        assert_eq!(intersects[0].t, 3.);
        assert_eq!(intersects[1].t, 7.);
    }

    #[test]
    pub fn intersect_translated_ray_with_sphere() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::static_default()
            .set_transform(&Matrix4::identity().translate(&Vector::new(5., 0., 0.)));
        let intersects = s.intersect(&r);
        assert!(intersects.is_none());
    }

    #[test_case(Point::new(1., 0., 0.), Vector::new(1., 0., 0.); "on x axis")]
    #[test_case(Point::new(0., 1., 0.), Vector::new(0., 1., 0.); "on y axis")]
    #[test_case(Point::new(0., 0., 1.), Vector::new(0., 0., 1.); "on z axis")]
    pub fn normal_at_point(p: Point, expected: Vector) {
        let s = Sphere::default();
        let n = s.get_normal(&p);
        assert_eq!(n, expected);
    }

    #[test]
    pub fn normal_at_nonaxial_point() {
        let s = Sphere::default();
        let p = Point::new(3_f32.sqrt() / 3., 3_f32.sqrt() / 3., 3_f32.sqrt() / 3.);
        let n = s.get_normal(&p);
        assert_eq!(
            n,
            Vector::new(3_f32.sqrt() / 3., 3_f32.sqrt() / 3., 3_f32.sqrt() / 3.)
        );
    }

    #[test]
    pub fn normal_is_normalized_vector() {
        let s = Sphere::default();
        let p = Point::new(3_f32.sqrt() / 3., 3_f32.sqrt() / 3., 3_f32.sqrt() / 3.);
        let n = s.get_normal(&p);
        assert_eq!(n, n.normalize());
    }

    #[test]
    pub fn normal_of_translated_sphere() {
        let s = Sphere::static_default()
            .set_transform(&Matrix4::identity().translate(&Vector::new(0., 1., 0.)));
        let n = s.get_normal(&Point::new(0., 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0., 0.70711, -0.70711));
    }

    #[test]
    pub fn normal_of_transformed_sphere() {
        let s = Sphere::static_default().set_transform(
            &Matrix4::identity()
                .rotate_z(std::f32::consts::PI / 5.)
                .scale(&Vector::new(1., 0.5, 1.)),
        );
        let n = s.get_normal(&Point::new(0., 2_f32.sqrt() / 2., -(2_f32.sqrt()) / 2.));
        assert_eq!(n, Vector::new(0., 0.97014, -0.24254));
    }
}
