use std::fmt::Debug;

use crate::{intersection::Intersection, ray::Ray};

pub trait Shape: Debug {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

impl PartialEq for dyn Shape {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<'a, T> Shape for &'a T
where
    T: Shape,
{
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        (*self).intersect(ray)
    }
}
