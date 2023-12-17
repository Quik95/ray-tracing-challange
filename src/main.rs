use std::env::{self};
use std::f32::consts::PI;

use itertools::Itertools;

use crate::matrix::Matrix4;
use crate::shape::{Cylinder, Plane};

use crate::tuple::{Color, Point, Vector};

use crate::camera::Camera;
use crate::light::PointLight;
use crate::material::Material;
use crate::pattern::{Checkers, Pattern, Stripe};

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

    let mut floor_pattern = Checkers::new(Color::new(0.5, 0.5, 0.5), Color::new(0.75, 0.75, 0.75));
    floor_pattern.set_transform(Matrix4::identity().scale(Vector::new(0.1, 0.1, 0.1)));
    let floor = Plane::default_with_material(Material {
        pattern: Some(floor_pattern),
        ..Default::default()
    })
    .set_transform(Matrix4::identity().scale(Vector::new(10., 0.01, 10.)));

    let mut backwall_pattern = Stripe::new(Color::new(0.5, 0.5, 0.5), Color::new(0.75, 0.75, 0.75));
    backwall_pattern.set_transform(Matrix4::identity().scale(Vector::new(0.1, 10., 0.1)));
    let backwall = Plane::default_with_material(Material {
        pattern: Some(backwall_pattern),
        ..Default::default()
    })
    .set_transform(
        Matrix4::identity()
            .translate(Vector::new(0., 0., 100.))
            .rotate_x(PI / 2.),
    );

    let cylinder1 = Cylinder::default_with_material(Material {
        pattern: Some(Box::new(Stripe {
            even: Color::new(0.5, 0.5, 0.5),
            odd: Color::new(0.75, 0.75, 0.75),
            transform: Matrix4::identity().scale(Vector::new(0.1, 0.1, 0.1)),
        })),
        ..Default::default()
    })
    .set_transform(
        Matrix4::identity()
            .rotate_x(PI / 3.)
            .rotate_y(-PI / 8.)
            .translate(Vector::new(-1.5, 2., -4.)),
    );
    cylinder1.minimum = -1.;
    cylinder1.maximum = 1.;

    let cylinder2 = Cylinder::default_with_material(Material {
        pattern: Some(Box::new(Stripe {
            even: Color::new(0.5, 0.5, 0.5),
            odd: Color::new(0.75, 0.75, 0.75),
            transform: Matrix4::identity().scale(Vector::new(0.1, 0.1, 0.1)),
        })),
        ..Default::default()
    })
    .set_transform(
        Matrix4::identity()
            .rotate_x(PI / 3.)
            .rotate_y(PI / 8.)
            .translate(Vector::new(1.5, 2., -4.)),
    );
    cylinder2.is_closed = true;
    cylinder2.minimum = -1.;
    cylinder2.maximum = 1.;

    let cylinder3 = Cylinder::default_with_material(Material {
        pattern: Some(Box::new(Stripe {
            even: Color::new(0.5, 0.5, 0.5),
            odd: Color::new(0.75, 0.75, 0.75),
            transform: Matrix4::identity().scale(Vector::new(0.1, 0.1, 0.1)),
        })),
        ..Default::default()
    })
    .set_transform(
        Matrix4::identity()
            .rotate_x(PI / 2.)
            .translate(Vector::new(0.0, 0.0, -5.)),
    );
    cylinder3.is_closed = true;
    cylinder3.minimum = -1.;
    cylinder3.maximum = 1.;

    let light_source = PointLight::new(Point::new(-5., 10., -20.), Color::new(1., 1., 1.));
    let world = world::World::new(
        light_source,
        vec![floor, cylinder1, cylinder2, cylinder3, backwall],
    );

    let mut camera = Camera::new(1920, 1080, PI / 3.);
    camera.set_transform(
        Point::new(0., 1.5, -10.),
        Point::new(0., 1., 0.),
        Vector::new(0., 1., 0.),
    );

    let canvas = camera.render(&world);

    let args = env::args().collect_vec();
    canvas.save_as_png(args.get(1).unwrap_or(&"./tracer.png".to_owned()))?;

    Ok(())
}
