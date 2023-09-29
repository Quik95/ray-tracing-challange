use std::f32::consts::PI;

use crate::matrix::Matrix4;
use crate::shape::{Cube, Plane, Shape};

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
            Color::new(0., 1., 0.),
            Color::new(1., 0.5, 0.),
        )),
        reflective: 0.4,
        transparency: 0.4,
        ..Default::default()
    });
    let mut backdrop_pattern =
        pattern::Checkers::new(Color::new(0.5, 0.5, 0.5), Color::new(0.75, 0.75, 0.75));
    backdrop_pattern.set_transform(
        &Matrix4::identity()
            .rotate_x(PI / 2.)
            .scale(&Vector::new(3., 3., 3.)),
    );
    let backdrop: &'static dyn Shape = Plane::default_with_material(Material {
        pattern: Some(backdrop_pattern),
        ..Default::default()
    })
    .set_transform(
        Matrix4::identity()
            .rotate_x(PI / 2.)
            .translate(&Vector::new(0., 0., 100.)),
    );

    let c1 = Cube::default_with_material(Material {
        reflective: 1.0,
        color: Color::new(0.5, 0.74, 0.12),
        ..Default::default()
    });
    c1.set_transform(Matrix4::identity().translate(&Vector::new(1.5, 1., 0.)));

    let c2 = Cube::default_with_material(Material {
        reflective: 1.0,
        color: Color::new(0.234, 0.315, 0.4168),
        ..Default::default()
    });
    c2.set_transform(Matrix4::identity().translate(&Vector::new(-1.5, 1., 0.)));

    let c3 = Cube::default_with_material(Material {
        reflective: 1.0,
        color: Color::new(0.3168, 0.6843, 0.354_318),
        ..Default::default()
    });
    c3.set_transform(Matrix4::identity().translate(&Vector::new(0.0, 3.5, 0.)));

    let light_source = PointLight::new(Point::new(-10., 1000., -1000.), Color::new(1., 1., 1.));
    let world = world::World::new(light_source, vec![floor, backdrop, c1, c2, c3]);

    let mut camera = Camera::new(1000, 1000, PI / 3.);
    camera.set_transform(
        Point::new(0., 1.5, -10.),
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
