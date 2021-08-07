use crate::ray::Ray;
use crate::shapes::Shape;

pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Intersection {
    pub fn new(t: f64, object: Shape) -> Self {
        Self {
            t: t,
            object: object,
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spheres::Sphere;

    #[test]
    fn test_encapsulation() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s.into());
        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, &s);
    }
}
