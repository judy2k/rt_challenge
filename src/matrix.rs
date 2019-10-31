use anyhow::Result;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let m = Matrix::new(2, 2);

        assert_eq!(m.value_at(0, 0), Some(&0.0));
        assert_eq!(m.value_at(0, 2), None);
        assert_eq!(m.value_at(2, 0), None);
        assert_eq!(m.value_at(2, 2), None);
    }
}
