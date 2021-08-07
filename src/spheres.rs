use crate::intersection::{Intersectable, Intersection};
use crate::ray::Ray;
use crate::shapes::Shape;
use crate::tuple::point;

#[derive(Debug, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin() - point(0., 0., 0.);
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = (b * b) - 4. * a * c;

        if discriminant < 0.0 {
            return vec![];
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);

            return vec![
                Intersection::new(t1, self.into()),
                Intersection::new(t2, self),
            ];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::{point, vector};

    #[test]
    fn test_intersect_sphere() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersects(s.into());
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_intersect_sphere_tangent() {
        let r = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersects(s.into());
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn test_intersect_sphere_miss() {
        let r = Ray::new(point(0., 2., -5.), vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersects(s.into());
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_sphere_from_center() {
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersects(s.into());
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_intersect_sphere_behind_ray() {
        let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = r.intersects(s.into());
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
}
