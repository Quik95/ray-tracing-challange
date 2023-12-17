use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::{Intersection, PrecomputedHit, Shape, Sphere};
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
        let s1 = Sphere::default_with_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let s2 = Sphere::static_default()
            .set_transform(Matrix4::identity().scale(Vector::new(0.5, 0.5, 0.5)));

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

    fn shade_hit(&self, comps: &PrecomputedHit, remaining_reflections: i32) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);

        let surface = self.light_source.calculate_lighting(
            comps.intersection.object.get_material(),
            comps.intersection.object,
            &comps.over_point,
            &comps.eye,
            &comps.normal,
            shadowed,
        );
        let reflected = self.reflected_color(comps, remaining_reflections);
        let refracted = self.refracted_color(comps, remaining_reflections);
        let material = comps.intersection.object.get_material();
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick_reflectance();
            return surface + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        surface + reflected + refracted
    }

    pub fn color_at(&self, r: &Ray, remaining_reflections: i32) -> Color {
        let xs = self.intersect_world(r);

        if let Some(hit) = Intersection::get_hit(&xs) {
            let comps = hit.precompute_hit(r, &xs);
            self.shade_hit(&comps, remaining_reflections)
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
    fn reflected_color(&self, comps: &PrecomputedHit, remaining_reflections: i32) -> Color {
        if remaining_reflections <= 0 {
            return Color::black();
        }

        if comps.intersection.object.get_material().reflective == 0.0 {
            return Color::black();
        }

        let reflected_ray = Ray::new(comps.over_point, comps.reflected_vector);
        let color = self.color_at(&reflected_ray, remaining_reflections - 1);
        color * comps.intersection.object.get_material().reflective
    }

    fn refracted_color(&self, comps: &PrecomputedHit, bounces_remaining: i32) -> Color {
        if bounces_remaining == 0 {
            return Color::black();
        }

        if comps.intersection.object.get_material().transparency == 0.0 {
            return Color::black();
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye.dot(&comps.normal);
        let sin2_t = n_ratio.powi(2) * cos_i.mul_add(-cos_i, 1.0);

        if sin2_t > 1.0 {
            return Color::black();
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normal * n_ratio.mul_add(cos_i, -cos_t) - comps.eye * n_ratio;
        let refracted_ray = Ray::new(comps.under_point, direction);
        let color = self.color_at(&refracted_ray, bounces_remaining - 1)
            * comps.intersection.object.get_material().transparency;

        color
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

        orientation * Self::identity().translate(Vector::new(-from.x, -from.y, -from.z))
    }
}

#[cfg(test)]
mod tests {
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::matrix::Matrix4;
    use crate::pattern::TestPattern;
    use crate::ray::Ray;
    use crate::shape::{Intersection, Plane, Shape, Sphere};
    use crate::tuple::{Color, Point, Vector};
    use crate::world::World;
    use nalgebra::matrix;
    use pretty_assertions::assert_eq;
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
        let i = crate::shape::Intersection::new(4., shape);
        let comps = i.precompute_hit(&r, &[i]);
        let c = w.shade_hit(&comps, 1);
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
        let i = crate::shape::Intersection::new(0.5, shape);
        let comps = i.precompute_hit(&r, &[i]);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, crate::tuple::Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    pub fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., -5.),
            crate::tuple::Vector::new(0., 1., 0.),
        );
        assert_eq!(w.color_at(&r, 1), Color::new(0., 0., 0.));
    }

    #[test]
    pub fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., -5.),
            crate::tuple::Vector::new(0., 0., 1.),
        );
        assert_eq!(w.color_at(&r, 1), Color::new(0.38066, 0.47582, 0.28549));
    }

    #[test]
    pub fn color_with_intersection_behind_ray() {
        let s1 = Sphere::default_with_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let s2 = Sphere::default_with_material(Material {
            ambient: 1.0,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().scale(Vector::new(0.5, 0.5, 0.5)));

        let w = World {
            objects: vec![s1, s2],
            ..Default::default()
        };
        let r = Ray::new(
            crate::tuple::Point::new(0., 0., 0.75),
            crate::tuple::Vector::new(0., 0., -1.),
        );
        assert_eq!(w.color_at(&r, 1), w.objects[1].get_material().color);
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
        assert_eq!(v, Matrix4::identity().scale(Vector::new(-1., 1., -1.)));
    }

    #[test]
    pub fn view_transform_moves_world() {
        let from = Point::new(0., 0., 8.);
        let to = Point::new(0., 0., 0.);
        let up = Vector::new(0., 1., 0.);
        let v = Matrix4::view_transform(from, to, up);
        assert_eq!(v, Matrix4::identity().translate(Vector::new(0., 0., -8.)));
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

    #[test_case(Point::new(0., 10., 0.), false; "point is not shadowed when nothing is collinear with point and light")]
    #[test_case(Point::new(10., - 10., 10.), true; "shadow when object is between point and light")]
    #[test_case(Point::new(- 20., 10., - 20.), false; "point is not shadowed when object is behind light")]
    #[test_case(Point::new(- 2., 2., - 2.), false; "no shadow when object is behind point")]
    pub fn no_shadow_when_nothing_is_collinear_with_point_and_light(p: Point, expected: bool) {
        let w = World::default();
        assert_eq!(w.is_shadowed(&p), expected);
    }

    #[test]
    pub fn shade_hit_in_shadow() {
        let s1 = Sphere::static_default();
        let s2 = Sphere::static_default()
            .set_transform(Matrix4::identity().translate(Vector::new(0., 0., 10.)));
        let light = PointLight::new(Point::new(0., 0., -10.), Color::new(1., 1., 1.));
        let w = World {
            objects: vec![s1, s2],
            light_source: light,
        };
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let i = Intersection::new(4., s2);
        let comps = i.precompute_hit(&r, &[i]);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    pub fn reflected_color_of_nonreflective_material() {
        let s1 = Sphere::default_with_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let s2 = Sphere::default_with_material(Material {
            ambient: 1.0,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().scale(Vector::new(0.5, 0.5, 0.5)));
        let w = World {
            objects: vec![s1, s2],
            ..Default::default()
        };
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let i = Intersection::new(1.0, s2);
        let comps = i.precompute_hit(&r, &[i]);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::black());
    }

    #[test]
    pub fn reflected_color_for_reflective_material() {
        let plane = Plane::default_with_material(Material {
            reflective: 0.5,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -1., 0.)));
        let mut w = World::default();
        w.objects.push(plane);

        let r = Ray::new(
            Point::new(0., 0., -3.),
            Vector::new(0., -(2.0_f32.sqrt()) / 2., (2.0_f32.sqrt()) / 2.),
        );
        let i = Intersection::new(2.0_f32.sqrt(), plane);
        let comps = i.precompute_hit(&r, &[i]);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.19033, 0.23791, 0.142_749));
    }

    #[test]
    pub fn shade_hit_with_reflective_material() {
        let plane = Plane::default_with_material(Material {
            reflective: 0.5,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -1., 0.)));
        let mut w = World::default();
        w.objects.push(plane);

        let r = Ray::new(
            Point::new(0., 0., -3.),
            Vector::new(0., -(2.0_f32.sqrt()) / 2., (2.0_f32.sqrt()) / 2.),
        );
        let i = Intersection::new(2.0_f32.sqrt(), plane);
        let comps = i.precompute_hit(&r, &[i]);
        let color = w.shade_hit(&comps, 1);
        assert_eq!(color, Color::new(0.87675, 0.92434, 0.82917));
    }

    #[test]
    pub fn color_at_with_mutually_reflective_surfaces() {
        let lower = Plane::default_with_material(Material {
            reflective: 1.0,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -1., 0.)));
        let upper = Plane::default_with_material(Material {
            reflective: 1.0,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., 1., 0.)));
        let w = World {
            objects: vec![lower, upper],
            light_source: PointLight::new(Point::new(0., 0., 0.), Color::new(1., 1., 1.)),
        };
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 1., 0.));
        let _ = w.color_at(&r, 1);
    }

    #[test]
    pub fn reflected_color_at_maximum_recursion_depth() {
        let plane = Plane::default_with_material(Material {
            reflective: 0.5,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -1., 0.)));
        let mut w = World::default();
        w.objects.push(plane);
        let r = Ray::new(
            Point::new(0., 0., -3.),
            Vector::new(0., -(2.0_f32.sqrt()) / 2., (2.0_f32.sqrt()) / 2.),
        );
        let i = Intersection::new(2.0_f32.sqrt(), plane);
        let comps = i.precompute_hit(&r, &[i]);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test_case(0, 1.0, 1.5)]
    #[test_case(1, 1.5, 2.0)]
    #[test_case(2, 2.0, 2.5)]
    #[test_case(3, 2.5, 2.5)]
    #[test_case(4, 2.5, 1.5)]
    #[test_case(5, 1.5, 1.0)]
    pub fn finding_n1_and_n2_at_various_intersections(index: usize, n1: f32, n2: f32) {
        let A = Sphere::static_glass_sphere();
        A.transform = Matrix4::identity().scale(Vector::new(2., 2., 2.));
        A.material.refractive_index = 1.5;

        let B = Sphere::static_glass_sphere();
        B.transform = Matrix4::identity().translate(Vector::new(0., 0., -0.25));
        B.material.refractive_index = 2.0;

        let C = Sphere::static_glass_sphere();
        C.transform = Matrix4::identity().translate(Vector::new(0., 0., 0.25));
        C.material.refractive_index = 2.5;

        let ray = Ray::new(Point::new(0., 0., -4.), Vector::new(0., 0., 1.));
        let xs = vec![
            Intersection::new(2.0, A),
            Intersection::new(2.75, B),
            Intersection::new(3.25, C),
            Intersection::new(4.75, B),
            Intersection::new(5.25, C),
            Intersection::new(6.0, A),
        ];
        let comps = xs[index].precompute_hit(&ray, &xs);
        assert_eq!(comps.n1, n1);
        assert_eq!(comps.n2, n2);
    }

    #[test]
    pub fn refracted_color_with_opaque_surface() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let w = World::default();
        let xs = vec![
            Intersection::new(4., w.objects[0]),
            Intersection::new(6., w.objects[0]),
        ];
        let comps = xs[0].precompute_hit(&r, &xs);
        let color = w.refracted_color(&comps, 5);
        assert_eq!(color, Color::black());
    }

    #[test]
    pub fn refracted_color_at_max_recursion_depth() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let objs = vec![Sphere::default_with_material(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        }) as &'static dyn Shape];
        let w = World {
            objects: objs,
            ..Default::default()
        };
        let xs = vec![
            Intersection::new(4., w.objects[0]),
            Intersection::new(6., w.objects[0]),
        ];
        let comps = xs[0].precompute_hit(&r, &xs);
        let color = w.refracted_color(&comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    pub fn refracted_color_under_total_internal_reflection() {
        let r = Ray::new(
            Point::new(0., 0., 2.0_f32.sqrt() / 2.),
            Vector::new(0., 1., 0.),
        );
        let objs = vec![Sphere::default_with_material(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        }) as &'static dyn Shape];
        let w = World {
            objects: objs,
            ..Default::default()
        };
        let sqrt2over2 = 2.0_f32.sqrt() / 2.;
        let xs = vec![
            Intersection::new(-sqrt2over2, w.objects[0]),
            Intersection::new(sqrt2over2, w.objects[0]),
        ];
        let comps = xs[1].precompute_hit(&r, &xs);
        let color = w.refracted_color(&comps, 5);
        assert_eq!(color, Color::black());
    }

    #[test]
    pub fn refracted_color_with_refracted_ray() {
        let r = Ray::new(Point::new(0., 0., 0.1), Vector::new(0., 1., 0.));
        let A = Sphere::default_with_material(Material {
            pattern: Some(TestPattern::new()),
            ambient: 1.0,
            ..Default::default()
        });
        let B = Sphere::default_with_material(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        });
        let w = World {
            objects: vec![A, B],
            ..Default::default()
        };

        let xs = vec![
            Intersection::new(-0.9899, A),
            Intersection::new(-0.4899, B),
            Intersection::new(0.4899, B),
            Intersection::new(0.9899, A),
        ];
        let comps = xs[2].precompute_hit(&r, &xs);
        let color = w.refracted_color(&comps, 5);
        assert_eq!(color, Color::new(0., 0.99887, 0.04721));
    }

    #[test]
    pub fn shade_hit_with_transparent_material() {
        let floor = Plane::default_with_material(Material {
            transparency: 0.5,
            refractive_index: 1.5,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -1., 0.)));
        let ball = Sphere::default_with_material(Material {
            color: Color::new(1., 0., 0.),
            ambient: 0.5,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -3.5, -0.5)));
        let w = World {
            objects: vec![floor, ball],
            ..Default::default()
        };
        let ray = Ray::new(
            Point::new(0., 0., -3.),
            Vector::new(0., -(2.0_f32.sqrt()) / 2., (2.0_f32.sqrt()) / 2.),
        );
        let i = Intersection::new(2.0_f32.sqrt(), floor);
        let comps = i.precompute_hit(&ray, &[i]);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    pub fn shade_hit_with_reflective_and_transparent_material() {
        let floor = Plane::default_with_material(Material {
            transparency: 0.5,
            refractive_index: 1.5,
            reflective: 0.5,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -1., 0.)));
        let sphere = Sphere::default_with_material(Material {
            color: Color::new(1., 0., 0.),
            ambient: 0.5,
            ..Default::default()
        })
        .set_transform(Matrix4::identity().translate(Vector::new(0., -3.5, -0.5)));
        let world = World {
            objects: vec![floor, sphere],
            ..Default::default()
        };
        let ray = Ray::new(
            Point::new(0., 0., -3.),
            Vector::new(0., -(2.0_f32.sqrt()) / 2., (2.0_f32.sqrt()) / 2.),
        );
        let i = vec![Intersection::new(2.0_f32.sqrt(), floor)];
        let comps = i[0].precompute_hit(&ray, &i);
        let color = world.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.92590, 0.686_425, 0.686_425));
    }
}
