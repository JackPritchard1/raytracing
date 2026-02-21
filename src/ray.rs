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
        blended_sky(self.direction)
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + t{} ", self.origin, self.direction)
    }
}

fn blended_sky(mut v: Vec3) -> Colour {
    let blue = v!(0.5, 0.7, 1.0);
    let white = v!(1.0, 1.0, 1.0);
    let v_norm = v.normalize();
    let t = (v_norm.y + 1.0)/ 2.0;
    white * (1.0 - t) + blue * t
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