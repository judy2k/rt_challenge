use anyhow::Result;

#[derive(Debug)]
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
        // TODO: Validate length of values.
        let mut m = Self::new(rows, cols);
        m.data = values.clone();
        Ok(m)
    }

    pub fn value_at(self: &Self, row: usize, col: usize) -> Option<&f64> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        self.data.get(self.cols * row + col)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.rows == other.rows && self.cols == other.cols {
            let pairs = self.data.iter().zip(other.data.iter());
            for (x, y) in pairs {
                if x != y {
                    return false;
                }
            }
            return true;
        }
        return false;
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

        assert_eq!(m.value_at(0, 0), Some(&1.0));
        assert_eq!(m.value_at(0, 3), Some(&4.0));
        assert_eq!(m.value_at(1, 0), Some(&5.5));
        assert_eq!(m.value_at(1, 2), Some(&7.5));
        assert_eq!(m.value_at(2, 2), Some(&11.));
        assert_eq!(m.value_at(3, 0), Some(&13.5));
        assert_eq!(m.value_at(3, 2), Some(&15.5));
        Ok(())
    }

    #[test]
    fn test_2x2_matrix() -> Result<()> {
        let m = Matrix::with_values(2, 2, vec![-3., 5., 1., -2.])?;
        assert_eq!(m.value_at(0, 0), Some(&-3.0));
        assert_eq!(m.value_at(0, 1), Some(&5.0));
        assert_eq!(m.value_at(1, 0), Some(&1.0));
        assert_eq!(m.value_at(1, 1), Some(&-2.0));

        assert_eq!(m.value_at(2, 0), None);
        assert_eq!(m.value_at(0, 2), None);

        Ok(())
    }

    #[test]
    fn test_3x3_matrix() -> Result<()> {
        let m = Matrix::with_values(3, 3, vec![-3., 5., 0., 1., -2., -7., 0., 1., 1.])?;
        assert_eq!(m.value_at(0, 0), Some(&-3.0));
        assert_eq!(m.value_at(1, 1), Some(&-2.0));
        assert_eq!(m.value_at(2, 2), Some(&1.0));

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
}
