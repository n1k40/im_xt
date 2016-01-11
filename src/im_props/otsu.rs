extern crate image;

use self::image::*;
use std::ops::{Add, Sub};
use im_props::histogram::Histogram;

trait otsu_threshold{
    fn get_otsu(&self) -> f32;
}

fn process(less_than_thresh: &Vec<(&u8, &i32)>, number_pixels: f32) -> (f32, f32) {
    let mut weight_V = Vec::<&u8>::new();
    let mut mean_V = Vec::<i32>::new();
    let mut denominator = 0;
    for &(pixel_value, count) in less_than_thresh {
        weight_V.push(pixel_value);
        let p = *pixel_value as i32;
        mean_V.push(p * count);
        denominator += *count;
    }
    let weight = weight_V.iter().map(|&p| *p as f32).fold(0., Add::add) / number_pixels;
    let mean = mean_V.iter().map(|&item| item as f32).fold(0., Add::add) / denominator as f32;
    let mut total = 0_f32;
    for &(pixel_value, count) in less_than_thresh {
        let x = *pixel_value as f32 - mean;
        let y = x * *count as f32;
        total += y;
    }
    let variance = total / denominator as f32;
    (weight, variance)
}

impl otsu_threshold for image::DynamicImage {
    fn get_otsu(&self) -> f32 {
        let histogram = Histogram::new(self);
        let (width, height) = self.dimensions();
        let number_pixels = (width * height) as f32;
        let mut values = Vec::<f32>::new();
        for threshold in 0..256 {
            let less_than_thresh = histogram.get_values_under_threshold(threshold);
            let (weight_b, variance_b) = process(&less_than_thresh,number_pixels);
            let more_than_thresh = histogram.get_values_over_threshold_inclusive(threshold);
            let (weight_f, variance_f) = process(&more_than_thresh, number_pixels);
            let x = (weight_b*variance_b) + (weight_f * variance_f);
            values.push(x);
        }
        values.iter().min().unwrap();        
    }
}
