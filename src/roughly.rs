const EPSILON: f64 = 0.00001;

pub trait RoughlyEqual {
    fn roughly_equal(self: &Self, other: &Self) -> bool;
}

impl RoughlyEqual for f64 {
    fn roughly_equal(self: &Self, other: &Self) -> bool {
        &(other + EPSILON) >= self && self >= &(other - EPSILON)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_f64_roughly_equal() {
        assert!(1.000005_f64.roughly_equal(&1.000000_f64))
    }
}
