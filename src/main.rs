use std::f32::consts::PI;

use crate::matrix::Matrix4;
use crate::shape::{Plane, Shape, Sphere};

use crate::tuple::{Color, Point, Vector};

use crate::camera::Camera;
use crate::light::PointLight;
use crate::material::Material;
use crate::pattern::Pattern;
use std::io;
use std::io::{BufWriter, Write};

mod camera;
mod canvas;
mod light;
mod material;
mod matrix;
mod pattern;
mod ray;
mod shape;
mod tuple;
mod world;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let _material = Material {
        color: Color::new(1., 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };
    let floor: &'static dyn Shape = Plane::default_with_material(Material {
        pattern: Some(pattern::Checkers::new(
            Color::new(1., 0., 0.),
            Color::black(),
        )),
        ..Default::default()
    });
    let _backdrop: &'static dyn Shape = Plane::default_with_material(Material {
        color: Color::new(1., 0.9, 0.9),
        specular: 0.0,
        pattern: Some(pattern::LinearGradient::new(
            Color::new(1.0, 0.0, 0.1),
            Color::new(0.0, 1.0, 0.1),
        )),
        ..Default::default()
    })
    .set_transform(
        Matrix4::identity()
            .rotate_x(PI / 2.)
            .translate(&Vector::new(0., 0., 10.)),
    );

    let mut middle_pattern =
        pattern::Ring::new(Color::new(1.0, 0.0, 0.1), Color::new(0.0, 1.0, 0.1));
    middle_pattern.set_transform(
        &Matrix4::identity()
            .rotate_x(PI / 2.)
            .scale(&Vector::new(0.1, 0.1, 0.1)),
    );
    let middle: &'static dyn Shape = Sphere::default_with_material(Material {
        color: Color::new(0.1, 1., 0.5),
        diffuse: 0.7,
        specular: 0.3,
        pattern: Some(middle_pattern),
        ..Default::default()
    })
    .set_transform(&Matrix4::identity().translate(&Vector::new(-0.5, 1., 0.5)));

    let right: &'static dyn Shape = Sphere::default_with_material(Material {
        color: Color::new(0.5, 1., 0.1),
        diffuse: 0.7,
        specular: 0.3,
        pattern: Some(pattern::LinearGradient::new(
            Color::new(1., 0., 0.),
            Color::new(0., 0., 1.),
        )),
        ..Default::default()
    })
    .set_transform(
        &Matrix4::identity()
            .scale(&Vector::new(0.5, 0.5, 0.5))
            .translate(&Vector::new(1.5, 0.5, -0.5)),
    );

    let left: &'static dyn Shape = Sphere::default_with_material(Material {
        color: Color::new(1., 0.8, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        pattern: None,
        ..Default::default()
    })
    .set_transform(
        &Matrix4::identity()
            .scale(&Vector::new(0.33, 0.33, 0.33))
            .translate(&Vector::new(-1.5, 0.33, -0.75)),
    );

    let left2: &'static dyn Shape = Sphere::default_with_material(Material {
        color: Color::new(0.420, 0.69, 0.2137),
        diffuse: 1.0,
        specular: 0.2,
        ..Default::default()
    })
    .set_transform(
        &Matrix4::identity()
            .scale(&Vector::new(0.33, 0.33, 0.33))
            .translate(&Vector::new(-0.5, 0., -1.75)),
    );

    let light_source = PointLight::new(Point::new(-10., 10., -10.), Color::new(1., 1., 1.));
    let world = world::World::new(light_source, vec![floor, middle, right, left, left2]);

    let mut camera = Camera::new(1000, 1000, PI / 3.);
    camera.set_transform(
        Point::new(0., 1.5, -5.),
        Point::new(0., 1., 0.),
        Vector::new(0., 1., 0.),
    );

    let canvas = camera.render(&world);

    let ppm = canvas.convert_to_ppm();
    dump_to_stdout(ppm.as_bytes())?;

    Ok(())
}

fn dump_to_stdout(data: &[u8]) -> color_eyre::Result<()> {
    let mut writer = BufWriter::new(io::stdout());
    writer.write_all(data)?;
    writer.flush()?;
    Ok(())
}
