use crate::canvas::Canvas;
use crate::tuple::{Point, Vector};
use std::io;
use std::io::{BufWriter, Write};

mod canvas;
mod matrix;
mod tuple;

struct Environment {
    gravity: Vector,
    wind: Vector,
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut canvas = Canvas::new(900, 550);

    let identity = Point::zero();
    let mut rotation = 0.0_f32;
    for _ in 0..12 {
        let p = identity
            .translate(&nalgebra::vector![0., canvas.center_point.y / 2.0, 0.])
            .rotate_z(rotation);

        rotation += std::f32::consts::PI / 6.0;
        canvas
            .draw_circle(
                p.x as i32 + canvas.center_point.x as i32,
                -p.y as i32 + canvas.center_point.y as i32,
                15,
            )
            .unwrap();
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
