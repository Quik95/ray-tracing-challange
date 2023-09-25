use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::objects::{Intersection, PrecomputedHit, Shape, Sphere};
use crate::ray::Ray;
use crate::tuple::{Color, Point, Vector};
use derive_more::Constructor;
use itertools::Itertools;
use nalgebra::matrix;

#[derive(Constructor)]
pub struct World {
    pub light_source: PointLight,
    pub objects: Vec<&'static dyn Shape>,
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere::default_with_material(Material::new(
            crate::tuple::Color::new(0.8, 1.0, 0.6),
            0.1,
            0.7,
            0.2,
            200.0,
            None,
        ));
        let s2 = Sphere::static_default()
            .set_transform(&Matrix4::identity().scale(&Vector::new(0.5, 0.5, 0.5)));

        Self {
            light_source: PointLight::new(
                crate::tuple::Point::new(-10., 10., -10.),
                crate::tuple::Color::new(1., 1., 1.),
            ),
            objects: vec![s1, s2],
        }
    }
}

impl World {
    fn intersect_world(&self, r: &Ray) -> Vec<Intersection> {
        self.objects
            .iter()
            .map(|&x| x.intersect(r))
            .filter(Option::is_some)
            .flatten()
            .flatten()
            .sorted()
            .collect_vec()
    }

    fn shade_hit(&self, comps: &PrecomputedHit) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);

        self.light_source.calculate_lighting(
            comps.intersection.object.get_material(),
            comps.intersection.object,
            &comps.over_point,
            &comps.eye,
            &comps.normal,
            shadowed,
        )
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        if let Some(hit) = Intersection::get_hit(&self.intersect_world(r)) {
            let comps = hit.precompute_hit(r);
            self.shade_hit(&comps)
        } else {
            Color::new(0., 0., 0.)
        }
    }

    pub fn is_shadowed(&self, p: &Point) -> bool {
        let v = self.light_source.position - p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*p, direction);
        let intersections = self.intersect_world(&r);
        let h = Intersection::get_hit(&intersections);

        h.is_some() && h.unwrap().t < distance
    }
}

impl Matrix4 {
    pub fn view_transform(
        from: crate::tuple::Point,
        to: crate::tuple::Point,
        up: crate::tuple::Vector,
    ) -> Self {
        let forward = (to - from).normalize();
        let up_normalized = up.normalize();
        let left = forward.cross(&up_normalized);
        let true_up = left.cross(&forward);

        let orientation: Self = matrix![
            left.x, left.y, left.z, 0.;
            true_up.x, true_up.y, true_up.z, 0.;
            -forward.x, -forward.y, -forward.z, 0.;
            0., 0., 0., 1.;
        ]
        .into();

        orientation * Self::identity().translate(&Vector::new(-from.x, -from.y, -from.z))
    }
}

#[cfg(test)]
mod tests {
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::matrix::Matrix4;
    use crate::objects::{Intersection, Shape, Sphere};
    use crate::ray::Ray;
    use crate::tuple::{Color, Point, Vector};
    use crate::world::World;
    use nalgebra::matrix;
    use test_case::test_case;

    #[test]
    pub fn intersect_world_with_ray() {
        let w = World::default();
        let r = crate::ray::Ray::new(
            crate::tuple::Point::new(0., 0., -5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let xs = w.intersect_world(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }

    #[test]
    pub fn shading_intersection() {
        let w = World::default();
        let r = crate::ray::Ray::new(
            crate::tuple::Point::new(0., 0., -5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let shape = w.objects[0];
        let i = crate::objects::Intersection::new(4., shape);
        let comps = i.precompute_hit(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, crate::tuple::Color::new(0.38066, 0.47582, 0.28549));
    }

    #[test]
    pub fn shading_intersection_from_inside() {
        let w = World {
            light_source: PointLight::new(
                crate::tuple::Point::new(0., 0.25, 0.),
                crate::tuple::Color::new(1., 1., 1.),
            ),
            ..Default::default()
        };
        let r = crate::ray::Ray::new(
            crate::tuple::Point::new(0., 0., 0.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        let shape = w.objects[1];
        let i = crate::objects::Intersection::new(0.5, shape);
        let comps = i.precompute_hit(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, crate::tuple::Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    pub fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., -5.),
            crate::tuple::Vector::new(0., 1., 0.),
        );
        assert_eq!(w.color_at(&r), Color::new(0., 0., 0.));
    }

    #[test]
    pub fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., -5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        assert_eq!(w.color_at(&r), Color::new(0.38066, 0.47582, 0.28549));
    }

    #[test]
    pub fn color_with_intersection_behind_ray() {
        let s1 = Sphere::default_with_material(Material::new(
            Color::new(0.8, 1.0, 0.6),
            1.0,
            0.7,
            0.2,
            200.0,
            None,
        ));
        let s2 = Sphere::default_with_material(Material {
            ambient: 1.0,
            ..Default::default()
        })
        .set_transform(&Matrix4::identity().scale(&Vector::new(0.5, 0.5, 0.5)));

        let w = World {
            objects: vec![s1, s2],
            ..Default::default()
        };
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., 0.75),
            crate::tuple::Vector::new(0., 0., -1.),
        );
        assert_eq!(w.color_at(&r), w.objects[1].get_material().color);
    }

    #[test]
    pub fn transformation_matrix_for_default_orientation() {
        let from = Point::new(0., 0., 0.);
        let up = Vector::new(0., 1., 0.);
        let to = Point::new(0., 0., -1.);
        let v = Matrix4::view_transform(from, to, up);
        assert_eq!(v, Matrix4::identity());
    }

    #[test]
    pub fn view_transform_looking_in_positive_z() {
        let from = Point::new(0., 0., 0.);
        let to = Point::new(0., 0., 1.);
        let up = Vector::new(0., 1., 0.);
        let v = Matrix4::view_transform(from, to, up);
        assert_eq!(v, Matrix4::identity().scale(&Vector::new(-1., 1., -1.)));
    }

    #[test]
    pub fn view_transform_moves_world() {
        let from = Point::new(0., 0., 8.);
        let to = Point::new(0., 0., 0.);
        let up = Vector::new(0., 1., 0.);
        let v = Matrix4::view_transform(from, to, up);
        assert_eq!(v, Matrix4::identity().translate(&Vector::new(0., 0., -8.)));
    }

    #[test]
    pub fn arbitrary_view_matrix() {
        let from = Point::new(1., 3., 2.);
        let to = Point::new(4., -2., 8.);
        let up = Vector::new(1., 1., 0.);
        let v = Matrix4::view_transform(from, to, up);

        let res: Matrix4 = matrix![
            -0.50709, 0.50709, 0.67612, -2.36643;
            0.76772, 0.60609, 0.12122, -2.82843;
            -0.35857, 0.59761, -0.71714, 0.00000;
            0.00000, 0.00000, 0.00000, 1.00000 ;
        ]
        .into();
        assert_eq!(v, res);
    }

    #[test_case(Point::new(0., 10., 0.), false ; "point is not shadowed when nothing is collinear with point and light")]
    #[test_case(Point::new(10., -10., 10.), true ; "shadow when object is between point and light")]
    #[test_case(Point::new(-20., 10., -20.), false ; "point is not shadowed when object is behind light")]
    #[test_case(Point::new(-2., 2., -2.), false ; "no shadow when object is behind point")]
    pub fn no_shadow_when_nothing_is_collinear_with_point_and_light(p: Point, expected: bool) {
        let w = World::default();
        assert_eq!(w.is_shadowed(&p), expected);
    }

    #[test]
    pub fn shade_hit_in_shadow() {
        let s1 = Sphere::static_default();
        let s2 = Sphere::static_default()
            .set_transform(&Matrix4::identity().translate(&Vector::new(0., 0., 10.)));
        let light = PointLight::new(Point::new(0., 0., -10.), Color::new(1., 1., 1.));
        let w = World {
            objects: vec![s1, s2],
            light_source: light,
        };
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let i = Intersection::new(4., s2);
        let comps = i.precompute_hit(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
