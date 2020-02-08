use rt_challenge::canvas::Canvas;
use rt_challenge::color::Color;
use rt_challenge::tuple::point;
use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;

fn main() -> std::io::Result<()> {
    let mut c = Canvas::new(400, 400);

    let tick_count = 12;
    for tick in 0..tick_count {
        let p = point(0., 150., 0.).rotate_z(tick as f64/tick_count as f64 * PI*2.).translate(200., 200., 0.);
        let cx = p.x() as i64;
        let cy = p.y() as i64;
        for dx in 0..3 {
            for dy in 0..3 {
                let px: i64 = cx + dx - 1;
                let py: i64 = cy + dy - 1;
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
