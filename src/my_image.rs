use image::{RgbImage};
use std::fs;
use crate::vector::*;

pub struct Image {
    pub rgb_image: RgbImage,
    pub aspect_ratio: f64,
    pub height: u32,
    pub width: u32,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64, viewport_height: f64, focal_length: f64 ) -> Self {
        let height = (width as f64 / aspect_ratio) as u32;
        let viewport_width = aspect_ratio * viewport_height;
        Self {
            rgb_image: RgbImage::new(width, height),
            aspect_ratio,
            height,
            width,
            viewport_height,
            viewport_width,
            focal_length,
        }
    }

    pub fn save_image(&self){
        let files = fs::read_dir("data").unwrap();
        let mut path = String::from("data/");
        path.push_str(files.count().to_string().as_str());
        path.push_str(".png");
        self.rgb_image.save(path).unwrap();
    }

    pub fn fill_red(&mut self){
        for x in 0..self.width {
            for y in 0..self.height {
                self.rgb_image.put_pixel(x, y, image::Rgb([255, 0, 0]));
            }
        }
    }

    pub fn gradient(&mut self){
        for x in 0..self.width {
            for y in 0..self.height {
                let rgb : Color = Color::new(x as f32 / self.width as f32, y as f32 / self.height as f32, 0.2);
                self.rgb_image.put_pixel(x, y, rgb.into())
            }
        }
    }

}





