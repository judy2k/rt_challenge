use super::tuple::Tuple;
use anyhow::{anyhow, Result};
use float_cmp::ApproxEqUlps;
use std::convert::TryInto;
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

    pub fn with_values(rows: usize, cols: usize, values: Vec<f64>) -> Result<Self> {
        if values.len() != rows * cols {
            return Err(anyhow!(
                "Length of values ({}) does not match matrix dimensions ({}x{})",
                values.len(),
                rows,
                cols,
            ));
        }
        let mut m = Self::new(rows, cols);
        m.data = values.clone();
        Ok(m)
    }

    pub fn value_at(self: &Self, row: usize, col: usize) -> Option<f64> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(self.data[self.cols * row + col])
    }

    pub fn set_value(self: &mut Self, row: usize, col: usize, value: f64) -> Result<()> {
        self.data[self.cols * row + col] = value;
        Ok(())
    }

    pub fn identity4() -> Matrix {
        Matrix::with_values(
            4,
            4,
            vec![
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        )
        .unwrap()
    }

    fn row(self: &Self, row: usize) -> Vec<f64> {
        (0..self.cols)
            .map(|col| self.value_at(row, col).expect("Out of bounds"))
            .collect()
    }

    fn col(self: &Self, col: usize) -> Vec<f64> {
        (0..self.rows)
            .map(|row| self.value_at(row, col).expect("Out of bounds"))
            .collect()
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
                result
                    .set_value(
                        col,
                        row,
                        self.value_at(row, col)
                            .expect(&format!("value_at: {}, {}", row, col)),
                    )
                    .expect(&format!("set_value: {}, {}", col, row));
            }
        }
        result
    }

    fn determinant(self: &Self) -> Result<f64> {
        if self.cols != 2 || self.rows != 2 {
            Err(anyhow!(
                "determinant is only possible for 2x2 matrices, not {}x{}",
                self.rows,
                self.cols
            ))
        } else {
            Ok(self.data[0] * self.data[3] - self.data[1] * self.data[2])
        }
    }

    fn submatrix(self: &Self, remove_row: usize, remove_col: usize) -> Result<Matrix> {
        if self.rows == 1 || self.cols == 1 {
            return Err(anyhow!(
                "Cannot generate a submatrix from a {}x{} matrix.",
                self.rows,
                self.cols
            ));
        }

        if remove_row >= self.rows {
            return Err(anyhow!(
                "Cannot remove row {} from a matrix with {} rows.",
                remove_row,
                self.rows
            ));
        }

        if remove_col >= self.cols {
            return Err(anyhow!(
                "Cannot remove col {} from a matrix with {} cols.",
                remove_col,
                self.cols
            ));
        }
        let mut result = Matrix::new(self.rows - 1, self.cols - 1);
        for row in 0..self.rows {
            if row != remove_row {
                for col in 0..self.cols {
                    if col != remove_col {
                        let dest_row = if row < remove_row { row } else { row - 1 };
                        let dest_col = if col < remove_col { col } else { col - 1 };
                        result
                            .set_value(dest_row, dest_col, self.value_at(row, col).unwrap())
                            .unwrap();
                    }
                }
            }
        }
        Ok(result)
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

impl Mul for Matrix {
    type Output = Result<Self>;
    fn mul(self: Self, rhs: Self) -> Result<Self> {
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
                result.set_value(row, col, Matrix::calculate_cell(row, col, &self, &rhs))?;
            }
        }

        Ok(result)
    }
}

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
                result.set_value(row, col, Matrix::calculate_cell(row, col, &self, &rhs))?;
            }
        }

        Ok(result)
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self: Self, t: Tuple) -> Tuple {
        return (self * Matrix::from(t)).unwrap().try_into().unwrap();
    }
}

impl From<Tuple> for Matrix {
    fn from(t: Tuple) -> Self {
        Matrix::with_values(4, 1, vec![t.x(), t.y(), t.z(), t.w()])
            .expect("Error creating Matrix from Tuple")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4_matrix() -> Result<()> {
        let m = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
            ],
        )?;

        assert_eq!(m.value_at(0, 0), Some(1.0));
        assert_eq!(m.value_at(0, 3), Some(4.0));
        assert_eq!(m.value_at(1, 0), Some(5.5));
        assert_eq!(m.value_at(1, 2), Some(7.5));
        assert_eq!(m.value_at(2, 2), Some(11.));
        assert_eq!(m.value_at(3, 0), Some(13.5));
        assert_eq!(m.value_at(3, 2), Some(15.5));
        Ok(())
    }

    #[test]
    fn test_2x2_matrix() -> Result<()> {
        let m = Matrix::with_values(2, 2, vec![-3., 5., 1., -2.])?;
        assert_eq!(m.value_at(0, 0), Some(-3.0));
        assert_eq!(m.value_at(0, 1), Some(5.0));
        assert_eq!(m.value_at(1, 0), Some(1.0));
        assert_eq!(m.value_at(1, 1), Some(-2.0));

        assert_eq!(m.value_at(2, 0), None);
        assert_eq!(m.value_at(0, 2), None);

        Ok(())
    }

    #[test]
    fn test_3x3_matrix() -> Result<()> {
        let m = Matrix::with_values(3, 3, vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.])?;
        assert_eq!(m.value_at(0, 0), Some(-3.0));
        assert_eq!(m.value_at(1, 1), Some(-2.0));
        assert_eq!(m.value_at(2, 2), Some(1.0));

        assert_eq!(m.value_at(3, 0), None);
        assert_eq!(m.value_at(0, 3), None);
        assert_eq!(m.value_at(3, 3), None);

        Ok(())
    }

    #[test]
    fn test_identical_matrices() -> Result<()> {
        let m1 = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
            ],
        )?;
        let m2 = Matrix::with_values(
            4,
            4,
            vec![
                1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
            ],
        )?;

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
        )?;
        let m2 = Matrix::with_values(
            4,
            4,
            vec![
                2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
            ],
        )?;

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
        )?;

        let m2 = Matrix::with_values(
            4,
            4,
            vec![
                -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
            ],
        )?;

        assert_eq!(
            (m1 * m2)?,
            Matrix::with_values(
                4,
                4,
                vec![
                    20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46.,
                    42.,
                ],
            )?
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
        )?;
        let t = Tuple::new(1., 2., 3., 1.);

        assert_eq!(m1 * t, Tuple::new(18., 24., 33., 1.));

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
        )?;
        assert_eq!((&m1 * &Matrix::identity4())?, m1);
        assert_eq!((m1.clone() * Matrix::identity4())?, m1);

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
        )?;

        let transposed = Matrix::with_values(
            4,
            4,
            vec![
                0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
            ],
        )?;

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
            Matrix::with_values(2, 2, vec![1., 5., -3., 2.])?.determinant()?,
            17.
        );
        Ok(())
    }

    #[test]
    fn test_submatrix() -> Result<()> {
        let m1 = Matrix::with_values(3, 3, vec![1., 5., 0., -3., 2., 7., 0., 6., -3.])?;
        let expected = Matrix::with_values(2, 2, vec![-3., 2., 0., 6.])?;

        assert_eq!(m1.submatrix(0, 2)?, expected);

        Ok(())
    }
}
