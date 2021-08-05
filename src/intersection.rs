use crate::shapes::Shape;

pub struct Intersection<'a> {
    pub t: f64,
    pub object: Box<dyn Shape + 'a>,
}

impl<'a> Intersection<'a> {
    pub fn new<T>(t: f64, object: T) -> Self
    where
        T: Shape + 'a,
    {
        Self {
            t: t,
            object: Box::new(object),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spheres::Sphere;

    #[test]
    fn test_encapsulation() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, s);
    }
}
