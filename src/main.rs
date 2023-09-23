use std::f32::consts::PI;

use crate::matrix::Matrix4;
use crate::objects::Sphere;

use crate::tuple::{Color, Point, Vector};

use crate::camera::Camera;
use crate::light::PointLight;
use crate::material::Material;
use std::io;
use std::io::{BufWriter, Write};

mod camera;
mod canvas;
mod light;
mod material;
mod matrix;
mod objects;
mod ray;
mod tuple;
mod world;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let material = Material {
        color: Color::new(1., 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };
    let floor = Sphere::default_with_material(material)
        .transform(&Matrix4::identity().scale(&Vector::new(10., 0.01, 10.)));

    let left_wall = Sphere::default_with_material(material).transform(
        &Matrix4::identity()
            .scale(&Vector::new(10., 0.01, 10.))
            .rotate_x(PI / 2.)
            .rotate_y(-PI / 4.)
            .translate(&Vector::new(0., 0., 5.)),
    );
    let right_wall = Sphere::default_with_material(material).transform(
        &Matrix4::identity()
            .scale(&Vector::new(10., 0.01, 10.))
            .rotate_x(PI / 2.)
            .rotate_y(PI / 4.)
            .translate(&Vector::new(0., 0., 5.)),
    );

    let middle = Sphere::default_with_material(Material {
        color: Color::new(0.1, 1., 0.5),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    })
    .transform(&Matrix4::identity().translate(&Vector::new(-0.5, 1., 0.5)));

    let right = Sphere::default_with_material(Material {
        color: Color::new(0.5, 1., 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    })
    .transform(
        &Matrix4::identity()
            .scale(&Vector::new(0.5, 0.5, 0.5))
            .translate(&Vector::new(1.5, 0.5, -0.5)),
    );

    let left = Sphere::default_with_material(Material {
        color: Color::new(1., 0.8, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    })
    .transform(
        &Matrix4::identity()
            .scale(&Vector::new(0.33, 0.33, 0.33))
            .translate(&Vector::new(-1.5, 0.33, -0.75)),
    );

    let light_source = PointLight::new(Point::new(-10., 10., -10.), Color::new(1., 1., 1.));
    let world = world::World::new(
        light_source,
        vec![floor, left_wall, right_wall, middle, right, left],
    );

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
