use image::RgbImage;
use crate::my_image::*;

mod my_image;
mod vector;
mod ray;

fn main() {
    let width = 400;
    let aspect_ratio = 16.0/9.0;
    let viewport_height = 2.0;
    let focal_length = 1.0;

    let mut image = my_image::Image::new(width, aspect_ratio, viewport_height, focal_length);
    image.gradient();
    image.save_image();


}
