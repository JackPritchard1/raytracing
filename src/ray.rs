use std::fmt;
use crate::vector::*;
use crate::v;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn colour(&self) -> Colour {
        v!(0.0,1.0,0.0)
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + t{} ", self.origin, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn ray_display() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(format!("{}", ray), "(0, 0, 0) + t(1, 0, 0) ");
    }
}