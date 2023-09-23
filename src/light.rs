use crate::material::Material;
use crate::tuple::{Color, Point, Vector};
use derive_more::Constructor;

#[derive(Constructor, Default, Copy, Clone, Eq, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn calculate_lighting(
        &self,
        material: &Material,
        pos: &Point,
        eye_vector: &Vector,
        normal_vector: &Vector,
        in_shadow: bool,
    ) -> Color {
        let diffuse;
        let specular;

        let effective_color = material.color * self.intensity;
        let ambient = effective_color * material.ambient;
        if in_shadow {
            return ambient;
        }

        let light_vector = (self.position - pos).normalize();
        let light_dot_normal = light_vector.dot(normal_vector);
        if light_dot_normal < 0. {
            diffuse = Color::new(0., 0., 0.);
            specular = Color::new(0., 0., 0.);
        } else {
            diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflect_vector = -light_vector.reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);

            if reflect_dot_eye < 0.0 {
                specular = Color::new(0., 0., 0.);
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = self.intensity * material.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::tuple::{Color, Point, Vector};
    use test_case::test_case;

    #[test_case(
    Vector::new(0., 0., -1.),
    Vector::new(0., 0., -1.),
    PointLight::new(Point::new(0., 0., -10.), Color::new(1., 1., 1.)),
    false,
    Color::new(1.9, 1.9, 1.9) ;
    "eye between light and surface, eye offset 45 degrees"
    )]
    #[test_case(
    Vector::new(0., 2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2.),
    Vector::new(0., 0., -1.),
    PointLight::new(Point::new(0., 0., -10.), Color::new(1., 1., 1.)),
    false,
    Color::new(1., 1., 1.) ;
    "eye between light and surface"
    )]
    #[test_case(
    Vector::new(0., 0., -1.),
    Vector::new(0., 0., -1.),
    PointLight::new(Point::new(0., 10., -10.), Color::new(1., 1., 1.)),
    false,
    Color::new(0.7364, 0.7364, 0.7364) ;
    "eye opposite surface, light offset 45 degrees"
    )]
    #[test_case(
    Vector::new(0., -(2.0_f32.sqrt()) / 2., -(2.0_f32.sqrt()) / 2.),
    Vector::new(0., 0., -1.),
    PointLight::new(Point::new(0., 10., -10.), Color::new(1., 1., 1.)),
    false,
    Color::new(1.63638, 1.63638, 1.63638) ;
    "eye in path of reflection vector"
    )]
    #[test_case(
    Vector::new(0., 0., -1.),
    Vector::new(0., 0., -1.),
    PointLight::new(Point::new(0., 0., 10.), Color::new(1., 1., 1.)),
    false,
    Color::new(0.1, 0.1, 0.1) ;
    "light behind a surface"
    )]
    pub fn eye_between_light_and_surface(
        eyev: Vector,
        normalv: Vector,
        light: PointLight,
        in_shadow: bool,
        expected: Color,
    ) {
        let position = Point::zero();
        let material = Material::default();
        let result = light.calculate_lighting(&material, &position, &eyev, &normalv, in_shadow);
        assert_eq!(result, expected);
    }
}
