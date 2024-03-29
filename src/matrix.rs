use super::roughly::RoughlyEqual;
use super::tuple::{Point, Vector};
use anyhow::{anyhow, Result};
use float_cmp::{ApproxEqUlps, Ulps};
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows * cols);
        data.resize(rows * cols, 0.0);
        Self { rows, cols, data }
    }

    pub fn with_values(rows: usize, cols: usize, values: Vec<f64>) -> Self {
        if values.len() != rows * cols {
            panic!(
                "Length of values ({}) does not match matrix dimensions ({}x{})",
                values.len(),
                rows,
                cols,
            );
        }
        let mut m = Self::new(rows, cols);
        m.data = values;
        m
    }

    pub fn value_at(self: &Self, row: usize, col: usize) -> f64 {
        assert!(
            row < self.rows,
            "row ({}) must be less than the number of rows ({})",
            row,
            self.rows
        );
        assert!(
            col < self.cols,
            "col ({}) must be less than the number of cols ({})",
            col,
            self.cols
        );
        self.data[self.cols * row + col]
    }

    pub fn set_value(self: &mut Self, row: usize, col: usize, value: f64) {
        self.data[self.cols * row + col] = value;
    }

    pub fn identity4() -> Self {
        Matrix::with_values(
            4,
            4,
            vec![
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        )
    }

    fn row(self: &Self, row: usize) -> Vec<f64> {
        (0..self.cols).map(|col| self.value_at(row, col)).collect()
    }

    fn col(self: &Self, col: usize) -> Vec<f64> {
        (0..self.rows).map(|row| self.value_at(row, col)).collect()
    }

    #[inline]
    fn calculate_cell(row: usize, col: usize, m1: &Matrix, m2: &Matrix) -> f64 {
        m1.row(row)
            .into_iter()
            .zip(m2.col(col).into_iter())
            .map(|(v1, v2)| v1 * v2)
            .sum::<f64>()
    }

    pub fn transpose(self: &Self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                result.set_value(col, row, self.value_at(row, col))
            }
        }
        result
    }

    fn determinant(self: &Self) -> f64 {
        if self.cols != 2 || self.rows != 2 {
            let mut det: f64 = 0.0;

            for col in 0..self.cols {
                det += self.value_at(0, col) * self.cofactor(0, col)
            }

            det
        } else {
            self.data[0] * self.data[3] - self.data[1] * self.data[2]
        }
    }

    fn submatrix(self: &Self, remove_row: usize, remove_col: usize) -> Matrix {
        if self.rows == 1 || self.cols == 1 {
            panic!(
                "Cannot generate a submatrix from a {}x{} matrix.",
                self.rows, self.cols
            );
        }

        if remove_row >= self.rows {
            panic!(
                "Cannot remove row {} from a matrix with {} rows.",
                remove_row, self.rows
            );
        }

        if remove_col >= self.cols {
            panic!(
                "Cannot remove col {} from a matrix with {} cols.",
                remove_col, self.cols
            );
        }
        let mut result = Matrix::new(self.rows - 1, self.cols - 1);
        for row in 0..self.rows {
            if row != remove_row {
                for col in 0..self.cols {
                    if col != remove_col {
                        let dest_row = if row < remove_row { row } else { row - 1 };
                        let dest_col = if col < remove_col { col } else { col - 1 };
                        result.set_value(dest_row, dest_col, self.value_at(row, col));
                    }
                }
            }
        }
        result
    }

    fn minor(self: &Self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(self: &Self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * if (row + col) % 2 == 1 { -1. } else { 1. }
    }

    fn invertible(self: &Self) -> bool {
        !self.determinant().approx_eq_ulps(&0.0, 2)
    }

    fn inverse(self: &Self) -> Matrix {
        if !self.invertible() {
            panic!("Cannot inverse uninvertible matrix.");
        } else {
            let self_determinant = self.determinant();
            let mut m2 = Matrix::new(self.rows, self.cols);

            for row in 0..self.rows {
                for col in 0..self.cols {
                    let c = self.cofactor(row, col);
                    m2.set_value(col, row, c / self_determinant);
                }
            }

            m2
        }
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Self::translation(x, y, z) * self
    }

    pub fn rotate_x(self, r: f64) -> Self {
        Self::rotation_x(r) * self
    }

    pub fn rotate_y(self, r: f64) -> Self {
        Self::rotation_y(r) * self
    }

    pub fn rotate_z(self, r: f64) -> Self {
        Self::rotation_z(r) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Self::scaling(x, y, z) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::shearing(xy, xz, yx, yz, zx, zy) * self
    }

    // FIXME: Need tests for translate, rotate_x, rotate_y, rotate_z, scale & shear.

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Matrix::with_values(
            4,
            4,
            vec![1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.],
        )
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Matrix::with_values(
            4,
            4,
            vec![x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 1.],
        )
    }

    pub fn rotation_x(r: f64) -> Self {
        Matrix::with_values(
            4,
            4,
            vec![
                1.,
                0.,
                0.,
                0.,
                0.,
                r.cos(),
                -r.sin(),
                0.,
                0.,
                r.sin(),
                r.cos(),
                0.,
                0.,
                0.,
                0.,
                1.,
            ],
        )
    }

    pub fn rotation_y(r: f64) -> Self {
        Matrix::with_values(
            4,
            4,
            vec![
                r.cos(),
                0.,
                r.sin(),
                0.,
                0.,
                1.,
                0.,
                0.,
                -r.sin(),
                0.,
                r.cos(),
                0.,
                0.,
                0.,
                0.,
                1.,
            ],
        )
    }

    pub fn rotation_z(r: f64) -> Self {
        Matrix::with_values(
            4,
            4,
            vec![
                r.cos(),
                -r.sin(),
                0.,
                0.,
                r.sin(),
                r.cos(),
                0.,
                0.,
                0.,
                0.,
                1.,
                0.,
                0.,
                0.,
                0.,
                1.,
            ],
        )
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::with_values(
            4,
            4,
            vec![
                1., xy, xz, 0., // Row 0
                yx, 1., yz, 0., // Row 1
                zx, zy, 1., 0., // Row 2
                0., 0., 0., 1., // Row 3
            ],
        )
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.rows == other.rows && self.cols == other.cols {
            let pairs = self.data.iter().zip(other.data.iter());
            for (x, y) in pairs {
                if !x.approx_eq_ulps(y, 2) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
}

impl RoughlyEqual for Matrix {
    fn roughly_equal(&self, other: &Matrix) -> bool {
        if self.rows == other.rows && self.cols == other.cols {
            let pairs = self.data.iter().zip(other.data.iter());
            for (x, y) in pairs {
                if !x.roughly_equal(y) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
}

impl ApproxEqUlps for Matrix {
    type Flt = f64;

    fn approx_eq_ulps(&self, other: &Self, ulps: <Self::Flt as Ulps>::U) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            false
        } else {
            self.data.iter().zip(other.data.iter()).all(|(a, b)| {
                print!("Comparing: {} with {}", a, b);
                a.approx_eq_ulps(b, ulps)
            })
        }
    }

    fn approx_ne_ulps(&self, other: &Self, ulps: <Self::Flt as Ulps>::U) -> bool {
        !self.approx_eq_ulps(other, ulps)
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self: Self, rhs: Self) -> Self {
        if self.cols != rhs.rows {
            panic!(
                "Matrix dimensions ({}, {}) and ({}, {}) are incompatible for multiplication.",
                self.rows, self.cols, rhs.rows, rhs.cols
            );
        }
        let mut result = Matrix::new(self.rows, rhs.cols);
        for row in 0..self.rows {
            for col in 0..rhs.cols {
                result.set_value(row, col, Matrix::calculate_cell(row, col, &self, &rhs));
            }
        }

        result
    }
}

// TODO: Do I need Mul implemented for &Matrix as well?
// FIXME: Should return Self, not Result<Matrix>
impl Mul for &Matrix {
    type Output = Result<Matrix>;
    fn mul(self: Self, rhs: Self) -> Result<Matrix> {
        if self.cols != rhs.rows {
            return Err(anyhow!(
                "Matrix dimensions ({}, {}) and ({}, {}) are incompatible for multiplication.",
                self.rows,
                self.cols,
                rhs.rows,
                rhs.cols
            ));
        }
        let mut result = Matrix::new(self.rows, rhs.cols);
        for row in 0..self.rows {
            for col in 0..rhs.cols {
                result.set_value(row, col, Matrix::calculate_cell(row, col, &self, &rhs));
            }
        }

        Ok(result)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self: Self, t: Self::Output) -> Self::Output {
        return (self * Matrix::from(t)).into();
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self: Self, t: Self::Output) -> Self::Output {
        return (self * Matrix::from(t)).into();
    }
}

impl From<Vector> for Matrix {
    fn from(t: Vector) -> Self {
        Matrix::with_values(4, 1, vec![t.x(), t.y(), t.z(), 0.0])
    }
}

impl From<Point> for Matrix {
    fn from(t: Point) -> Self {
        Matrix::with_values(4, 1, vec![t.x(), t.y(), t.z(), 1.0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};
    use std::f64::consts::PI;

    /// Check if two floats are approximately equal
    macro_rules! assert_float_eq {
        ($left: expr, $right: expr) => {
            assert!($left.roughly_equal(&$right));
        };
    }

    #[test]
    fn test_4x4_matrix() -> Result<()> {
        let m = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
            ],
        );

        assert_eq!(m.value_at(0, 0), 1.0);
        assert_eq!(m.value_at(0, 3), 4.0);
        assert_eq!(m.value_at(1, 0), 5.5);
        assert_eq!(m.value_at(1, 2), 7.5);
        assert_eq!(m.value_at(2, 2), 11.);
        assert_eq!(m.value_at(3, 0), 13.5);
        assert_eq!(m.value_at(3, 2), 15.5);
        Ok(())
    }

    #[test]
    fn test_2x2_matrix() {
        let m = Matrix::with_values(2, 2, vec![-3., 5., 1., -2.]);
        assert_eq!(m.value_at(0, 0), -3.0);
        assert_eq!(m.value_at(0, 1), 5.0);
        assert_eq!(m.value_at(1, 0), 1.0);
        assert_eq!(m.value_at(1, 1), -2.0);
    }

    #[test]
    #[should_panic]
    fn test_2x2_matrix_bounds_panic1() {
        let m = Matrix::with_values(2, 2, vec![-3., 5., 1., -2.]);

        m.value_at(2, 0);
    }

    #[test]
    #[should_panic]
    fn test_2x2_matrix_bounds_panic2() {
        let m = Matrix::with_values(2, 2, vec![-3., 5., 1., -2.]);

        let v = m.value_at(0, 2);
        println!("V is {}", v);
    }

    #[test]
    #[should_panic]
    fn test_2x2_matrix_bounds_panic3() {
        let m = Matrix::with_values(2, 2, vec![-3., 5., 1., -2.]);

        m.value_at(2, 2);
    }

    #[test]
    fn test_3x3_matrix() {
        let m = Matrix::with_values(3, 3, vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.]);
        assert_eq!(m.value_at(0, 0), -3.0);
        assert_eq!(m.value_at(1, 1), -2.0);
        assert_eq!(m.value_at(2, 2), 1.0);
    }

    #[test]
    #[should_panic]
    fn test_3x3_matrix_bounds_panic1() {
        let m = Matrix::with_values(3, 3, vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.]);

        m.value_at(3, 0);
    }

    #[test]
    #[should_panic]
    fn test_3x3_matrix_bounds_panic2() {
        let m = Matrix::with_values(3, 3, vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.]);

        m.value_at(0, 3);
    }

    #[test]
    #[should_panic]
    fn test_3x3_matrix_bounds_panic3() {
        let m = Matrix::with_values(3, 3, vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.]);

        m.value_at(3, 3);
    }

    #[test]
    fn test_identical_matrices() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
            ],
        );
        let m2 = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
            ],
        );

        assert_eq!(m1, m2);
        Ok(())
    }

    #[test]
    fn test_different_matrices() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
            ],
        );
        let m2 = Matrix::with_values(
            4,
            4,
            vec![
                2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
            ],
        );

        assert_ne!(m1, m2);
        Ok(())
    }

    #[test]
    fn test_matrix_multiplication() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
            ],
        );

        let m2 = Matrix::with_values(
            4,
            4,
            vec![
                -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
            ],
        );

        assert_eq!(
            (m1 * m2),
            Matrix::with_values(
                4,
                4,
                vec![
                    20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46.,
                    42.,
                ],
            )
        );

        Ok(())
    }

    #[test]
    fn matrix_multiplied_with_tuple() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
            ],
        );
        let t = Point::new(1., 2., 3.);

        assert_eq!(m1 * t, Point::new(18., 24., 33.));

        Ok(())
    }

    #[test]
    fn matrix_multiplied_with_identity() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                0., 1., 2., 4., 1., 2., 4., 8., 2., 4., 8., 16., 4., 8., 16., 32.,
            ],
        );
        assert_eq!((&m1 * &Matrix::identity4())?, m1);
        assert_eq!((m1.clone() * Matrix::identity4()), m1);

        Ok(())
    }

    #[test]
    fn matrix_transposition() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
            ],
        );

        let transposed = Matrix::with_values(
            4,
            4,
            vec![
                0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
            ],
        );

        assert_eq!(m1.transpose(), transposed);

        Ok(())
    }

    #[test]
    fn transpose_identity_matrix() {
        assert_eq!(Matrix::identity4().transpose(), Matrix::identity4());
    }

    #[test]
    fn test_determinant() -> Result<()> {
        assert_eq!(
            Matrix::with_values(2, 2, vec![1., 5., -3., 2.]).determinant(),
            17.
        );
        Ok(())
    }

    #[test]
    fn test_submatrix_3x3() -> Result<()> {
        let m1 = Matrix::with_values(3, 3, vec![1., 5., 0., -3., 2., 7., 0., 6., -3.]);
        let expected = Matrix::with_values(2, 2, vec![-3., 2., 0., 6.]);

        assert_eq!(m1.submatrix(0, 2), expected);

        Ok(())
    }

    #[test]
    fn test_submatrix_4x4() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                -6., 1., 1., 6., -8., 5., 8., 6., -1., 0., 8., 2., -7., 1., -1., 1.,
            ],
        );
        let expected = Matrix::with_values(3, 3, vec![-6., 1., 6., -8., 8., 6., -7., -1., 1.]);

        assert_eq!(m1.submatrix(2, 1), expected);

        Ok(())
    }

    #[test]
    fn test_minor() -> Result<()> {
        let a = Matrix::with_values(3, 3, vec![3., 5., 0., 2., -1., -7., 6., -1., 5.]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.);
        assert_eq!(a.minor(1, 0), 25.);

        Ok(())
    }

    #[test]
    fn test_cofactor() -> Result<()> {
        let a = Matrix::with_values(3, 3, vec![3., 5., 0., 2., -1., -7., 6., -1., 5.]);
        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);

        Ok(())
    }

    #[test]
    fn test_determinant_3x3() {
        let a = Matrix::with_values(3, 3, vec![1., 2., 6., -5., 8., -4., 2., 6., 4.]);
        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn test_determinant_4x4() {
        let a = Matrix::with_values(
            4,
            4,
            vec![
                -2., -8., 3., 5., -3., 1., 7., 3., 1., 2., -9., 6., -6., 7., 7., -9.,
            ],
        );
        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn test_invertible() {
        let a = Matrix::with_values(
            4,
            4,
            vec![
                6., 4., 4., 4., 5., 5., 7., 6., 4., -9., 3., -7., 9., 1., 7., -6.,
            ],
        );
        assert_eq!(a.determinant(), -2120.);
        assert_eq!(a.invertible(), true);
    }

    #[test]
    fn test_notinvertible() {
        let a = Matrix::with_values(
            4,
            4,
            vec![
                -4., 2., -2., -3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., 0.,
            ],
        );
        assert_eq!(a.determinant(), 0.);
        assert_eq!(a.invertible(), false);
    }

    #[test]
    fn test_inverse() {
        let a = Matrix::with_values(
            4,
            4,
            vec![
                -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
            ],
        );
        let b = a.inverse();

        assert_eq!(a.determinant(), 532.);
        assert_float_eq!(a.cofactor(2, 3), -160.);
        assert_float_eq!(b.value_at(3, 2), -160. / 532.);
        assert_float_eq!(a.cofactor(3, 2), 105.);
        assert_float_eq!(b.value_at(2, 3), 105. / 532.);

        assert_float_eq!(
            b,
            Matrix::with_values(
                4,
                4,
                vec![
                    0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                    -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639
                ]
            )
        );
    }

    #[test]
    fn test_approx_eq() {
        println!("Ulps: {}", 0.21804511278195488_f64.ulps(&0.21805_f64));
        assert_float_eq!(0.21804511278195488_f64, 0.21805_f64);
        println!("Ulps: {}", -0.045112781954887216_f64.ulps(&-0.04511_f64));
        assert_float_eq!(-0.045112781954887216_f64, -0.04511_f64);
    }

    #[test]
    fn test_translation() {
        let transform = Matrix::translation(5., -3., 2.);
        let p = point(-3., 4., 5.);
        assert_eq!(transform * p, point(2., 1., 7.));
    }

    #[test]
    fn test_translation_inverse() {
        let transform = Matrix::translation(5., -3., 2.).inverse();
        let p = point(-3., 4., 5.);
        assert_eq!(transform * p, point(-8., 7., 3.));
    }

    #[test]
    fn test_translation_doesnt_affect_vectors() {
        let transform = Matrix::translation(5., -3., 2.);
        let v = vector(-3., 4., 5.);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn test_scaling_point() {
        let transform = Matrix::scaling(2., 3., 4.);
        let p = point(-4., 6., 8.);
        assert_eq!(transform * p, point(-8., 18., 32.));
    }

    #[test]
    fn test_scaling_vector() {
        let transform = Matrix::scaling(2., 3., 4.);
        let v = vector(-4., 6., 8.);
        assert_eq!(transform * v, vector(-8., 18., 32.));
    }

    #[test]
    fn test_scaling_inverse() {
        let transform = Matrix::scaling(2., 3., 4.).inverse();
        let v = vector(-4., 6., 8.);
        assert_eq!(transform * v, vector(-2., 2., 2.));
    }

    #[test]
    fn test_reflection() {
        let transform = Matrix::scaling(-1., 1., 1.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(-2., 3., 4.));
    }

    #[test]
    fn test_rotation_x() {
        let p = point(0., 1., 0.);
        let half_quarter = Matrix::rotation_x(PI / 4.);
        let full_quarter = Matrix::rotation_x(PI / 2.);
        assert_eq!(
            half_quarter * p,
            point(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
        );
        assert_eq!(full_quarter * p, point(0., 0., 1.));
    }

    #[test]
    fn test_rotation_x_inverse() {
        let p = point(0., 1., 0.);
        let half_quarter = Matrix::rotation_x(PI / 4.).inverse();
        assert_eq!(
            half_quarter * p,
            point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)
        );
    }

    #[test]
    fn test_rotation_y() {
        let p = point(0., 0., 1.);
        let half_quarter = Matrix::rotation_y(PI / 4.);
        let full_quarter = Matrix::rotation_y(PI / 2.);
        assert_eq!(
            half_quarter * p,
            point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.)
        );
        assert_eq!(full_quarter * p, point(1., 0., 0.));
    }

    #[test]
    fn test_rotation_z() {
        let p = point(0., 1., 0.);
        let half_quarter = Matrix::rotation_z(PI / 4.);
        let full_quarter = Matrix::rotation_z(PI / 2.);
        assert_eq!(
            half_quarter * p,
            point(-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.)
        );
        assert_eq!(full_quarter * p, point(-1., 0., 0.));
    }

    #[test]
    fn test_shearing_xy() {
        let t = Matrix::shearing(1., 0., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(t * p, point(5., 3., 4.))
    }

    #[test]
    fn test_shearing_xz() {
        let t = Matrix::shearing(0., 1., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(t * p, point(6., 3., 4.))
    }

    #[test]
    fn test_shearing_yx() {
        let t = Matrix::shearing(0., 0., 1., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(t * p, point(2., 5., 4.))
    }

    #[test]
    fn test_shearing_yz() {
        let t = Matrix::shearing(0., 0., 0., 1., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(t * p, point(2., 7., 4.))
    }

    #[test]
    fn test_shearing_zx() {
        let t = Matrix::shearing(0., 0., 0., 0., 1., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(t * p, point(2., 3., 6.))
    }

    #[test]
    fn test_shearing_zy() {
        let t = Matrix::shearing(0., 0., 0., 0., 0., 1.);
        let p = point(2., 3., 4.);
        assert_eq!(t * p, point(2., 3., 7.))
    }

    #[test]
    fn test_transformations_in_sequence() {
        let p = point(1., 0., 1.);
        let a = Matrix::rotation_x(PI / 2.);
        let b = Matrix::scaling(5., 5., 5.);
        let c = Matrix::translation(10., 5., 7.);

        let p2 = a * p;
        assert_eq!(p2, point(1., -1., 0.));

        let p3 = b * p2;
        assert_eq!(p3, point(5., -5., 0.));

        let p4 = c * p3;
        assert_eq!(p4, point(15., 0., 7.));
    }

    #[test]
    fn test_chained_transformation_multiplication() {
        let p = point(1., 0., 1.);
        let a = Matrix::rotation_x(PI / 2.);
        let b = Matrix::scaling(5., 5., 5.);
        let c = Matrix::translation(10., 5., 7.);

        let t = c * b * a;
        assert_eq!(t * p, point(15., 0., 7.));
    }

    #[test]
    fn test_chained_transformation_calls() {
        let p = point(1., 0., 1.);
        let t = Matrix::identity4()
            .rotate_x(PI / 2.)
            .scale(5., 5., 5.)
            .translate(10., 5., 7.);

        assert_eq!(t * p, point(15., 0., 7.));
    }
}
