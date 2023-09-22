use crate::canvas::Canvas;
use crate::matrix::Matrix4;
use crate::objects::Hittable;
use crate::ray::Ray;
use crate::tuple::{Color, Point};

use crate::light::PointLight;
use crate::material::Material;
use std::io;
use std::io::{BufWriter, Write};

mod canvas;
mod light;
mod material;
mod matrix;
mod objects;
mod ray;
mod tuple;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let canvas_pixels = 1000;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let t = Matrix4::identity();
    let sphere = objects::Sphere::default_with_material(Material {
        color: Color::new(1., 0.2, 1.),
        ..Default::default()
    })
    .transform(&t);
    let light = PointLight::new(Point::new(-10., 10., -10.), Color::new(1., 1., 1.));

    let ray_origin = Point::new(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = pixel_size / 2.;

    for y in (-canvas.center_point.y as i32)..(canvas.center_point.y as i32) {
        let world_y = half - pixel_size * y as f32;
        for x in (-canvas.center_point.x as i32)..(canvas.center_point.x as i32) {
            let world_x = -half + pixel_size * x as f32;
            let p = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (p - ray_origin).normalize());
            if let Some(xs) = sphere.intersect(&r) {
                let hit = xs.get_hit().unwrap();
                let point = r.position(hit);
                let normal = xs.object.get_normal(&point);
                let eye = -r.direction;
                let color =
                    light.calculate_lighting(xs.object.get_material(), &point, &eye, &normal);
                canvas.write_pixel(x, -y, color).unwrap();
            }
        }
    }

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
