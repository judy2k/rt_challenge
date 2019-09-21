
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
        writeln!(&mut result, "p3");
        writeln!(
            &mut result,
            "{width} {height}",
            width = self.width,
            height = self.height
        );
        writeln!(&mut result, "255");

        return result;
    }
}

#[cfg(test)]
mod canvas_tests {
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
    fn canvas_to_ppm() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let mut ppm_lines = ppm.lines();
        assert_eq!(Some("p3"), ppm_lines.next());
        assert_eq!(Some("5 3"), ppm_lines.next());
        assert_eq!(Some("255"), ppm_lines.next());
    }
}