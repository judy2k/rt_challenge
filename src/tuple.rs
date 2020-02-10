use crate::matrix::Matrix;
use crate::roughly::RoughlyEqual;
use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Tuple([f64; 4]);

#[derive(Copy, Clone)]
pub struct Point(Tuple);

#[derive(Copy, Clone)]
pub struct Vector(Tuple);

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        return Tuple([x, y, z, w]);
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    #[inline]
    pub fn w(&self) -> f64 {
        self.0[3]
    }

    pub fn is_point(&self) -> bool {
        return self.w() == 1.0;
    }

    pub fn is_vector(&self) -> bool {
        return self.w() == 0.0;
    }
}

impl From<Matrix> for Point {
    fn from(m: Matrix) -> Self {
        Self(m.into())
    }
}

impl From<Matrix> for Vector {
    fn from(m: Matrix) -> Self {
        Self(m.into())
    }
}

impl From<Matrix> for Tuple {
    fn from(m: Matrix) -> Self {
        Tuple::new(
            m.value_at(0, 0),
            m.value_at(1, 0),
            m.value_at(2, 0),
            m.value_at(3, 0),
        )
    }
}

impl fmt::Debug for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tuple({:?}, {:?}, {:?}, {:?})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
            self.w() + other.w(),
        )
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Point(self.0 + other.0)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Vector(self.0 + other.0)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
            self.w() - other.w(),
        )
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector(self.0 - other.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point(self.0 - other.0)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector(self.0 - other.0)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
            self.w() * rhs,
        )
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
            self.w() / rhs,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x().roughly_equal(&other.x())
            && self.y().roughly_equal(&other.y())
            && self.z().roughly_equal(&other.z())
            && self.w().roughly_equal(&other.w())
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(Tuple::new(x, y, z, 1.0))
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0.x()
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0.y()
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.0.z()
    }

    pub fn rotate_x(self, r: f64) -> Self {
        Matrix::rotation_x(r) * self
    }

    pub fn rotate_y(self, r: f64) -> Self {
        Matrix::rotation_y(r) * self
    }

    pub fn rotate_z(self, r: f64) -> Self {
        Matrix::rotation_z(r) * self
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::scaling(x, y, z) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vector({:?}, {:?}, {:?})",
            self.0.x(),
            self.0.y(),
            self.0.z()
        )
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point({:?}, {:?}, {:?})",
            self.0.x(),
            self.0.y(),
            self.0.z()
        )
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Self(Tuple::new(x, y, z, 0.0))
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0.x()
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0.y()
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.0.z()
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.0.x().powi(2) + self.0.y().powi(2) + self.0.z().powi(2) + self.0.w().powi(2)).sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Vector {
        Self(self.0 / self.magnitude())
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        return self.0.x() * other.0.x()
            + self.0.y() * other.0.y()
            + self.0.z() * other.0.z()
            + self.0.w() * other.0.w();
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> Vector {
        return Self::new(
            self.0.y() * other.0.z() - self.0.z() * other.0.y(),
            self.0.z() * other.0.x() - self.0.x() * other.0.z(),
            self.0.x() * other.0.y() - self.0.y() * other.0.x(),
        );
    }

    pub fn rotate_x(self, r: f64) -> Self {
        Matrix::rotation_x(r) * self
    }

    pub fn rotate_y(self, r: f64) -> Self {
        Matrix::rotation_y(r) * self
    }

    pub fn rotate_z(self, r: f64) -> Self {
        Matrix::rotation_z(r) * self
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::scaling(x, y, z) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Point {
    return Point::new(x, y, z);
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    return Vector::new(x, y, z);
}

impl From<Tuple> for Vector {
    fn from(t: Tuple) -> Self {
        Self(t)
    }
}

impl From<Tuple> for Point {
    fn from(t: Tuple) -> Self {
        Self(t)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEqUlps;
    use std::f64::consts::PI;

    #[cfg(test)]
    /// Check if two floats are approximately equal
    macro_rules! assert_float_eq {
        ($left: expr, $right: expr, $precision: expr) => {
            assert!($left.approx_eq_ulps(&$right, $precision));
        };
    }

    #[test]
    /// Testing a point tuple
    fn w1_tuple_is_a_point() {
        let t = super::Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_float_eq!(4.3, t.x(), 2);
        assert_float_eq!(-4.2, t.y(), 2);
        assert_float_eq!(3.1, t.z(), 2);
        assert_float_eq!(1.0, t.w(), 2);

        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    /// Testing a vector tuple
    fn w0_tuple_is_a_vector() {
        let t = super::Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_float_eq!(4.3, t.x(), 2);
        assert_float_eq!(-4.2, t.y(), 2);
        assert_float_eq!(3.1, t.z(), 2);
        assert_float_eq!(0.0, t.w(), 2);

        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    /// Adding two Tuples
    fn add_tuples() {
        let a1 = super::Tuple::new(3.0, -2., 5., 1.);
        let a2 = super::Tuple::new(-2., 3., 1., 0.);

        assert_eq!(a1 + a2, super::Tuple::new(1., 1., 6., 1.))
    }

    #[test]
    /// Subtracting two points
    fn subtract_points() {
        let a1 = super::point(3., 2., 1.);
        let a2 = super::point(5., 6., 7.);

        assert_eq!(a1 - a2, super::vector(-2., -4., -6.))
    }

    #[test]
    /// Subtracting a vector from a point
    fn subtract_vector_from_point() {
        let p = super::point(3., 2., 1.);
        let v = super::vector(5., 6., 7.);

        assert_eq!(p - v, super::point(-2., -4., -6.))
    }

    #[test]
    /// Subtracting two vectors
    fn subtract_two_vectors() {
        let v1 = super::vector(3., 2., 1.);
        let v2 = super::vector(5., 6., 7.);
        assert_eq!(v1 - v2, super::vector(-2., -4., -6.));
    }

    #[test]
    /// Comparing two tuples for equality
    fn compare_tuples() {
        assert_eq!(
            super::Tuple::new(2., 3., -4., 0.4 * 0.1),
            super::Tuple::new(2., 3., -4., 0.04),
        );
    }

    #[test]
    /// Subtracting a vector from the zero vector
    fn subtract_from_zero_vector() {
        let zero = super::vector(0., 0., 0.);
        let v = super::vector(1., -2., 3.);
        assert_eq!(zero - v, super::vector(-1., 2., -3.));
    }

    #[test]
    /// Negating a tuple
    fn negate_tuple() {
        let a = super::Tuple::new(1., -2., 3., -4.);
        assert_eq!(-a, super::Tuple::new(-1., 2., -3., 4.));
    }

    #[test]
    /// Multiplying a tuple by a scalar
    fn multiply_tuple_and_scalar() {
        let a = super::Tuple::new(1., -2., 3., -4.);
        assert_eq!(a * 3.5_f64, super::Tuple::new(3.5, -7., 10.5, -14.));
    }

    #[test]
    /// Multiplying a tuple by a fraction
    fn multiply_tuple_and_fraction() {
        let a = super::Tuple::new(1., -2., 3., -4.);
        assert_eq!(a * 0.5, super::Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    /// Dividing a tuple by a scalar
    fn divide_tuple_by_scalar() {
        let a = super::Tuple::new(1., -2., 3., -4.);
        assert_eq!(a / 2.0, super::Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn magnitude_tests() {
        assert_eq!(super::vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(super::vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(super::vector(1.0, 2.0, 3.0).magnitude(), (14.0_f64).sqrt());
        assert_eq!(
            super::vector(-1.0, -2.0, -3.0).magnitude(),
            (14.0_f64).sqrt()
        );
    }

    #[test]
    fn normalization_tests() {
        assert_eq!(
            super::vector(4., 0., 0.).normalize(),
            super::vector(1., 0., 0.)
        );
        assert_eq!(
            super::vector(1., 2., 3.).normalize(),
            super::vector(1. / 14_f64.sqrt(), 2. / 14_f64.sqrt(), 3. / 14_f64.sqrt())
        );
        assert_eq!(super::vector(1., 2., 3.).normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_test() {
        let a = super::vector(1., 2., 3.);
        let b = super::vector(2., 3., 4.);

        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_test() {
        let a = super::vector(1., 2., 3.);
        let b = super::vector(2., 3., 4.);
        assert_eq!(a.cross(&b), super::vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), super::vector(1., -2., 1.));
    }

    #[test]
    fn test_chained_transformation_calls() {
        let p = super::point(1., 0., 1.)
            .rotate_x(PI / 2.)
            .scale(5., 5., 5.)
            .translate(10., 5., 7.);

        assert_eq!(p, super::point(15., 0., 7.));
    }
}
