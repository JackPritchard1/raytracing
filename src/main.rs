use image::RgbImage;
use crate::my_image::*;
use crate::ray::*;
use crate::vector::*;

mod my_image;
mod vector;
mod ray;

fn main() {
    let mut image = Image::initialise_default();
    image.gradient();
    image.save_image();

    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
}

