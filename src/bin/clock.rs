use rt_challenge::canvas::Canvas;
use rt_challenge::color::Color;
use rt_challenge::tuple::{point};
use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;
use rt_challenge::matrix::Matrix;


fn main() -> std::io::Result<()> {
    let mut c = Canvas::new(800, 800);

    for tick in 0..12 {
        let t = Matrix::identity4().rotate_z(tick as f64/12. * PI*2.).translate(400., 400., 0.);
        let p = t * point(0., 300., 0.);
        let cx = p.x() as usize;
        let cy = p.y() as usize;
        for dx in 0..3 {
            for dy in 0..3 {
                let px: i64 = (cx + dx) as i64 - 1;
                let py: i64 = (cy + dy) as i64 - 1;
                if px >= 0 && px < 800 && py >= 0 && py < 800 {
                    c.set_pixel(px as usize, py as usize, Color::new(1.0, 0.5, 0.5));
                }
            }
        }
    }
    println!("Writing 'clock.ppm'");
    let mut file = File::create("clock.ppm")?;
    file.write_all(&c.to_ppm().into_bytes())?;
    println!("Done.");
    Ok(())
}
