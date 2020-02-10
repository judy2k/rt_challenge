use crate::tuple::{Point, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray{origin, direction}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};

    #[test]
    fn test_ray_construction() {
        let ray = Ray::new(point(1., 2., 3.), vector(4., 5., 6.));
        assert_eq!(ray.origin, point(1., 2., 3.));
        assert_eq!(ray.direction, vector(4., 5., 6.));
    }
}