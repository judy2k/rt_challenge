use crate::color::Color;
use std::fmt::Write;

pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, Color::new(0., 0., 0.));

        return Self {
            width,
            height,
            data,
        };
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        return self.data.get(self.coords_to_index(x, y));
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, value: Color) {
        let idx = self.coords_to_index(x, y);
        self.data[idx] = value;
    }

    #[inline]
    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn to_ppm(&self) -> String {
        self.try_to_ppm()
            .expect("Writing to String should never fail.")
    }

    pub fn try_to_ppm(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        writeln!(&mut result, "P3")?;
        writeln!(
            &mut result,
            "{width} {height}",
            width = self.width,
            height = self.height
        )?;
        writeln!(&mut result, "255")?;

        let mut line = String::with_capacity(self.width() * self.height() * 5);
        for row in 0..self.height() {
            line.clear();
            for col in 0..self.width() {
                let pix = self.pixel_at(col, row).unwrap();
                let rgb = vec![pix.red(), pix.green(), pix.blue()];
                for i in rgb.into_iter() {
                    let s = clamp_byte(i).to_string();
                    if line.len() + s.len() >= 70 {
                        result.write_str(&line)?;
                        result.write_str("\n")?;
                        line.clear();
                        line.write_str(&s)?;
                    } else {
                        if line.len() > 0 {
                            line.write_str(" ")?;
                        }
                        line.write_str(&s)?;
                    }
                }
            }
            if line.len() > 0 {
                result.write_str(&line)?;
                result.write_str("\n")?;
            }
        }

        return Ok(result);
    }
}

fn clamp_byte(val: f64) -> u8 {
    let result = (val * 255.).round().max(0.).min(255.);
    result.round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_new() {
        let c = super::Canvas::new(10, 20);
        assert_eq!(10_usize, c.width());
        assert_eq!(20_usize, c.height());

        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(&Color::new(0., 0., 0.), c.pixel_at(x, y).unwrap());
            }
        }
    }

    #[test]
    fn writing_to_canvas() {
        let mut c = super::Canvas::new(10, 20);
        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(&Color::new(0., 0., 0.), c.pixel_at(x, y).unwrap());
            }
        }

        c.set_pixel(3, 14, Color::new(1.0, 0.5, 0.25));

        for x in 0..10 {
            for y in 0..20 {
                if x == 3 && y == 14 {
                    assert_eq!(&Color::new(1.0, 0.5, 0.25), c.pixel_at(x, y).unwrap());
                } else {
                    assert_eq!(&Color::new(0., 0., 0.), c.pixel_at(x, y).unwrap());
                }
            }
        }
    }

    #[test]
    fn canvas_to_ppm_header() {
        let c = Canvas::new(5, 3);

        let ppm = c.to_ppm();
        let mut ppm_lines = ppm.lines();
        assert_eq!(Some("P3"), ppm_lines.next());
        assert_eq!(Some("5 3"), ppm_lines.next());
        assert_eq!(Some("255"), ppm_lines.next());
    }

    #[test]
    fn small_canvas_to_ppm() {
        let mut c = Canvas::new(5, 3);
        c.set_pixel(0, 0, Color::new(1.5, 0., 0.));
        c.set_pixel(2, 1, Color::new(0., 0.5, 0.));
        c.set_pixel(4, 2, Color::new(-0.5, 0., 1.));

        let ppm = c.to_ppm();
        let line_vec: Vec<&str> = ppm.lines().collect();
        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", line_vec[3]);
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", line_vec[4]);
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", line_vec[5]);
    }

    #[test]
    fn canvas_splits_long_lines() {
        let mut c = Canvas::new(10, 2);
        for x in 0..c.width {
            for y in 0..c.height {
                c.set_pixel(x, y, Color::new(1., 0.8, 0.6));
            }
        }

        let ppm = c.to_ppm();
        let line_vec: Vec<&str> = ppm.lines().collect();
        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            line_vec[3]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            line_vec[4]
        );
        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            line_vec[5]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            line_vec[6]
        );
    }

    #[test]
    fn ppm_ends_with_newline() {
        let c = Canvas::new(5, 3);

        let ppm = c.to_ppm();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }

    #[test]
    fn test_clamp_byte() {
        assert_eq!(128, clamp_byte(0.5));
        assert_eq!(255, clamp_byte(1.5));
        assert_eq!(255, clamp_byte(1.0));
        assert_eq!(0, clamp_byte(0.0));
        assert_eq!(0, clamp_byte(-0.5));
        assert_eq!(0, clamp_byte(-1.0));
        assert_eq!(0, clamp_byte(-1.5));
    }
}
