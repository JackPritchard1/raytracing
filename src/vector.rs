use image::Rgb;
use derive_more::{Add, Sub, Mul, Div, Neg};
use std::fmt;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq, Add, Sub, Mul, Div, Neg, PartialOrd)]
pub(crate) struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Self::new(x, y, z)
    }
    
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn len(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn map(self, f: fn(f32) -> f32) -> Self{
        let x = f(self.x);
        let y = f(self.y);
        let z = f(self.z);
        Self::new(x, y, z)
    }

    pub fn vec_mul(&self, other: &Self) -> Self{
        Self::new(self.x*other.x, self.y*other.y, self.z*other.z)
    }

}

pub type Color = Vec3;
pub type Point = Vec3;

impl From<Vec3> for Rgb<u8>{
    fn from(v: Vec3) -> Self {
        Rgb([(v.x*255.0) as u8, (v.y*255.0) as u8, (v.z*255.0) as u8])
    }
}

impl Mul<Vec3> for f32{
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[macro_export]
macro_rules! v {
    ($x:expr, $y: expr, $z: expr) => {
        Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
    ($x:expr) => {
        Vec3::new(f64::from($x), f64::from($x), f64::from($x))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a.cross(&b), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_len() {
        let a = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(a.len(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let mut a = Vec3::new(3.0, 4.0, 0.0);
        a.normalize();
        assert_eq!(a, Vec3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn test_map() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.map(|x| x*x), Vec3::new(1.0, 4.0, 9.0));
    }

    #[test]
    fn test_color(){
        let c = Color::new(0.5, 0.5, 0.0);
        let rgb = Rgb([127, 127, 0]);
        let colour: Rgb<u8> = c.into();
        assert_eq!(colour, rgb);
    }

    #[test]
    fn test_vec_addition(){
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a+b, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec_subtraction(){
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a-b, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_vec_multiplication(){
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b: f32 = 3.0;
        assert_eq!(a*b, Vec3::new(3.0, 6.0, 9.0));
        assert_eq!(b*a, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vec_division(){
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b: f32 = 2.0;
        assert_eq!(a/b, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_vec_negation(){
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
    }


}