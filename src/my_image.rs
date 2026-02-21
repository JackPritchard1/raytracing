use image::{RgbImage};
use std::fs;
use crate::my_image;
use crate::ray::Ray;
use crate::vector::*;
use crate::v;

pub struct Image {
    pub rgb_image: RgbImage,
    pub aspect_ratio: f64,
    pub height: Vec3,
    pub width: Vec3,
    pub focal_length: Vec3,
}

impl Image {
    pub fn new(height: Vec3, aspect_ratio: f64, image_width: u32, focal_length: Vec3) -> Self {
        let width = v!(height.len() * aspect_ratio, 0.0, 0.0);
        let image_height = (image_width as f64/aspect_ratio) as u32;
        println!("image_width: {}, image_height: {}", image_width, image_height);
        println!("aspect_ratio: {}", aspect_ratio);
        println!("height: {}, width: {}", height, width);
        println!("focal_length: {}", focal_length);
        println!("---------------------------------------");

        Self {
            rgb_image: RgbImage::new(image_width, image_height),
            aspect_ratio,
            height,
            width,
            focal_length,
        }
    }

    pub fn initialise_default() -> Self {
        let height = v!(0.0,2.0,0.0);
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let focal_length = v!(0.0,0.0,1.0);
        Image::new(height, aspect_ratio, image_width, focal_length)
    }

    pub fn save_image(&self) {
        let files = fs::read_dir("data").unwrap();
        let mut path = String::from("data/");
        path.push_str(files.count().to_string().as_str());
        path.push_str(".png");
        self.rgb_image.save(path).unwrap();
    }

    pub fn fill_red(&mut self) {
        for x in 0..self.rgb_image.width() as i32 {
            for y in 0..self.rgb_image.height() as i32 {
                self.rgb_image.put_pixel(x as u32, y as u32, image::Rgb([255, 0, 0]));
            }
        }
    }

    pub fn gradient(&mut self) {
        for x in 0..self.rgb_image.width() {
            for y in 0..self.rgb_image.height() {
                let rgb: Colour = Colour::new(x as f64 / self.rgb_image.width() as f64, y as f64 / self.rgb_image.height() as f64, 0.2);
                self.rgb_image.put_pixel(x, y, rgb.into())
            }
        }
    }

    pub fn ray_trace(&mut self) {
        let origin = v!(0.0,0.0,0.0);
        let top_left_corner = origin - (self.width / 2.0) + (self.height / 2.0) - self.focal_length;
        let width = self.rgb_image.width();
        let height = self.rgb_image.height();
        for j in 0..height {
            for i in 0..width {
                let v = top_left_corner + (i as f64/width as f64) * self.width - (j as f64 /height as f64) * self.height - origin;
                let ray = Ray::new(origin, v);
                let colour = ray.colour();
                self.rgb_image.put_pixel(i, j, colour.into());
            }
        }
    }
}






