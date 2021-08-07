use std::fmt::Debug;

use crate::{intersection::Intersectable, intersection::Intersection, ray::Ray, spheres::Sphere};

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl Intersectable for Shape {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match *self {
            Shape::Sphere(ref sphere) => sphere.intersect(ray),
        }
    }
}

impl From<Sphere> for Shape {
    fn from(a: Sphere) -> Shape {
        Shape::Sphere(a)
    }
}
