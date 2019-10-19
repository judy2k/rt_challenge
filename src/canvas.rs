
use super::color::Color;
use std::fmt::Write;

struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, Color::new(0., 0., 0.));

        return Self {
            width,
            height,
            data,
        };
    }

    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    #[inline]
    fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        return self.data.get(self.coords_to_index(x, y));
    }

    #[inline]
    fn set_pixel(&mut self, x: usize, y: usize, value: Color) {
        let idx = self.coords_to_index(x, y);
        self.data[idx] = value;
    }

    #[inline]
    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn to_ppm(&self) -> String {
        let mut result = String::new();
        writeln!(&mut result, "P3").unwrap();
        writeln!(
            &mut result,
            "{width} {height}",
            width = self.width,
            height = self.height
        )
        .unwrap();
        writeln!(&mut result, "255").unwrap();

        for row in 0..self.height() {
            let mut line = String::new();
            for col in 0..self.width() {
                let pix = self.pixel_at(col, row).unwrap();
                for i in vec![pix.red(), pix.green(), pix.blue()] {
                    let s = format!("{}", clamp_byte(i));
                    if line.len() + s.len() > 70 {
                        println!("Mid-row break after: {}", line);
                        write!(&mut result, "{}", line).unwrap();
                        line.clear();
                        writeln!(&mut line, "{}", s).unwrap();
                    } else {
                        if line.len() > 0 {
                            write!(&mut line, " ").unwrap();
                        }
                        write!(&mut line, "{}", s).unwrap();
                    }
                }
            }
            if line.len() > 0 {
                println!("Printing line: \"{}\"", line);
                writeln!(&mut result, "{}", line).unwrap();
            }
        }

        return result;
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

