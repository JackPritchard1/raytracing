use image::RgbImage;
use crate::my_image::*;
use crate::ray::*;
use crate::vector::*;

mod my_image;
mod vector;
mod ray;

fn main() {
    let mut image = Image::initialise_default();
    image.ray_trace();
    image.save_image();
}

