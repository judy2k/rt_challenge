use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Mul, Sub};

extern crate float_cmp;
use float_cmp::ApproxEqUlps;

#[derive(Clone)]
pub struct Color(pub [f64; 3]);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        return Color([r, g, b]);
    }

    #[inline]
    pub fn red(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn green(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn blue(&self) -> f64 {
        self.0[2]
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Color({:?}, {:?}, {:?})",
            self.red(),
            self.green(),
            self.blue(),
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red().approx_eq_ulps(&other.red(), 2)
            && self.green().approx_eq_ulps(&other.green(), 2)
            && self.blue().approx_eq_ulps(&other.blue(), 2)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.red() + other.red(),
            self.green() + other.green(),
            self.blue() + other.blue(),
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color::new(
            self.red() - other.red(),
            self.green() - other.green(),
            self.blue() - other.blue(),
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.red() * other.red(),
            self.green() * other.green(),
            self.blue() * other.blue(),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Color::new(
            self.red() * other,
            self.green() * other,
            self.blue() * other,
        )
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn new_color() {
        let c = super::Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn compare_colors() {
        let c1 = super::Color::new(0.9, 0.6, 0.75);
        let c2 = super::Color::new(0.9, 0.6, 0.75);
        assert_eq!(c1, c2);
    }

    #[test]
    fn add_colors() {
        let c1 = super::Color::new(0.9, 0.6, 0.75);
        let c2 = super::Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, super::Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let c1 = super::Color::new(0.9, 0.6, 0.75);
        let c2 = super::Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, super::Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = super::Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2., super::Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_colors() {
        let c1 = super::Color::new(1., 0.2, 0.4);
        let c2 = super::Color::new(0.9, 1., 0.1);
        assert_eq!(c1 * c2, super::Color::new(0.9, 0.2, 0.04));
    }
}