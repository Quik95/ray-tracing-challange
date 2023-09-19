use crate::tuple::{Point, Vector};

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

fn main() {
    let mut p = Projectile {
        velocity: Vector::new(1., 1., 0.).normalize(),
        position: Point::new(0., 1., 0.),
    };
    let e = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };
    let mut n_ticks = 0;
    while p.position.y > 0.0 {
        p = tick(&e, &p);
        n_ticks += 1;
        println!("{p:?}");
    }
    println!("It took {n_ticks} for the projectile to fall to the ground.");
}
