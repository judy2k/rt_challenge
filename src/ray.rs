use crate::intersection::Intersectable;
use crate::intersection::Intersection;
use crate::shapes::Shape;
use crate::tuple::{Point, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vector {
        self.direction.clone()
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn intersects(&self, shape: Shape) -> Vec<Intersection> {
        shape.intersect(self)
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

    #[test]
    fn test_position() {
        let r = Ray::new(point(2., 3., 4.), vector(1., 0., 0.));
        assert_eq!(r.position(0.0), point(2., 3., 4.));
        assert_eq!(r.position(1.0), point(3., 3., 4.));
        assert_eq!(r.position(-1.0), point(1., 3., 4.));
        assert_eq!(r.position(2.5), point(4.5, 3., 4.));
    }
}
