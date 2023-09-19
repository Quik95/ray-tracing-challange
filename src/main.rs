use crate::canvas::Canvas;
use crate::tuple::{Point, Vector};
use std::io;
use std::io::{BufWriter, Write};

mod canvas;
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

    let mut p = Projectile {
        velocity: Vector::new(1., 2.8, 0.).normalize() * 11.25,
        position: Point::new(0., 1., 0.),
    };
    let e = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };
    let mut canvas = Canvas::new(1920, 550);
    let mut n_ticks = 0;
    while p.position.y > 0.0 {
        p = tick(&e, &p);
        n_ticks += 1;
        let r = canvas.write_pixel(
            p.position.x as u32,
            550_u32.saturating_sub(p.position.y as u32),
            tuple::Color::new(1., 0., 0.),
        );
        match r {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Out of bounds coordinate: {e}");
            }
        }
    }
    eprintln!("It took {n_ticks} for the projectile to fall to the ground.");
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
