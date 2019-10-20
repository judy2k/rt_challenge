use rt_challenge::canvas::Canvas;
use rt_challenge::color::Color;
use rt_challenge::tuple::{point, vector, Tuple};
use std::fs::File;
use std::io::Write;

struct Env {
    gravity: Tuple,
    wind: Tuple,
}

impl Env {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        return Self { gravity, wind };
    }
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        return Self { position, velocity };
    }
}

fn tick(env: &Env, proj: Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    return Projectile::new(pos, velocity);
}

fn main() -> std::io::Result<()> {
    let start = point(0., 1., 0.);
    let velocity = vector(1., 1.8, 0.).normalize() * 11.25;
    let mut p = Projectile::new(start, velocity);

    let gravity = vector(0., -0.1, 0.);
    let wind = vector(-0.01, 0., 0.);
    let e = Env::new(gravity, wind);

    let mut c = Canvas::new(900, 550);

    while p.position.y() > 0.0 {
        let cx = p.position.x() as usize;
        let cy = c.height() - p.position.y() as usize;
        for dx in 0..3 {
            for dy in 0..3 {
                let px: i64 = (cx + dx) as i64 - 1;
                let py: i64 = (cy + dy) as i64 - 1;
                if px >= 0 && px < 900 && py >= 0 && py < 550 {
                    c.set_pixel(px as usize, py as usize, Color::new(1.0, 0.5, 0.5));
                }
            }
        }
        p = tick(&e, p);
    }
    println!("Writing 'projectile.ppm'");
    let mut file = File::create("projectile.ppm")?;
    file.write_all(&c.to_ppm().into_bytes())?;
    println!("Done.");
    Ok(())
}
