use rt_challenge::tuple::{Tuple, vector, point};


struct Env {
    gravity: Tuple,
    wind: Tuple,
}

impl Env {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        return Self{
            gravity, wind,
        }
    }
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        return Self{
            position, velocity,
        };
    }
}

fn tick(env: Env, proj: Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    return Projectile::new(pos, velocity);
}

#[test]
fn ticking() {
    let e = Env::new(vector(0., -0.1, 0.), vector(-0.01, 0., 0.));
    let p = Projectile::new(point(0., 1., 0.), vector(1., 1., 0.).normalize());
}