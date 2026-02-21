use image::{RgbImage};
use std::fs;
use crate::my_image;
use crate::vector::*;
use crate::v;

pub struct Image {
    pub rgb_image: RgbImage,
    pub aspect_ratio: f64,
    pub height: Vec3,
    pub width: Vec3,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: Vec3,
}

impl Image {
    pub fn new(width: Vec3, aspect_ratio: f64, viewport_height: f64, focal_length: Vec3 ) -> Self {
        let height = width / aspect_ratio;
        let viewport_width = aspect_ratio * viewport_height;
        Self {
            rgb_image: RgbImage::new(width.len() as u32, height.len() as u32),
            aspect_ratio,
            height,
            width,
            viewport_height,
            viewport_width,
            focal_length,
        }
    }

    pub fn initialise_default() -> Self{
        let width = v!(400,0,0);
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let focal_length = v!(0.0,0,1.0);
        Image::new(width, aspect_ratio, viewport_height, focal_length)
    }

    pub fn save_image(&self){
        let files = fs::read_dir("data").unwrap();
        let mut path = String::from("data/");
        path.push_str(files.count().to_string().as_str());
        path.push_str(".png");
        self.rgb_image.save(path).unwrap();
    }

    pub fn fill_red(&mut self){
        for x in 0..self.width.len() as i32 {
            for y in 0..self.height.len() as i32 {
                self.rgb_image.put_pixel(x as u32, y as u32, image::Rgb([255, 0, 0]));
            }
        }
    }

    pub fn gradient(&mut self){
        for x in 0..self.width.len() as u32 {
            for y in 0..self.height.len() as u32 {
                let rgb : Colour = Colour::new(x as f64 / self.width.len() as f64, y as f64 / self.height.len() as f64, 0.2);
                self.rgb_image.put_pixel(x, y, rgb.into())
            }
        }
    }

}





