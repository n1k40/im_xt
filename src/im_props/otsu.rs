extern crate image;

use self::image::*;
use std::ops::{Add};

use im_props::histogram::Histogram;

pub trait OtsuThreshold {
    fn get_otsu(&self) -> (f32, usize);
}

fn process(less_than_thresh: &Vec<(&u8, &i32)>, number_pixels: f32) -> (f32, f32) {
    let mut weight_v = Vec::<&i32>::new();
    let mut mean_v = Vec::<i32>::new();
    let mut denominator = 0;
    for &(pixel_value, count) in less_than_thresh {
        println!("pixel: {}, count: {}", pixel_value, count);
        weight_v.push(count);
        let p = *pixel_value as i32;
        mean_v.push(p * count);
        denominator += *count;
    }

    let weight = weight_v.iter().map(|&p| *p as f32).fold(0., Add::add) / number_pixels;
    let mean = mean_v.iter().map(|&item| item as f32).fold(0., Add::add) / denominator as f32;
    let mut total = 0_f32;
    for &(pixel_value, count) in less_than_thresh {
        let x = *pixel_value as f32 - mean;
        let xx = x * x;
        let y = xx * *count as f32;
        total += y;
    }
    let variance = total / denominator as f32;
    //println!("variance is: {}", variance);
    (weight, variance)
}

impl OtsuThreshold for DynamicImage {
    fn get_otsu(&self) -> (f32, usize) {
        let histogram = Histogram::new(self);
        let (width, height) = self.dimensions();
        let number_pixels = (width * height) as f32;
        let mut within_class_variances = Vec::<f32>::new();
        for threshold in 0..255 {
            let less_than_thresh = histogram.get_values_under_threshold(threshold);
            let (weight_b, variance_b) = process(&less_than_thresh,number_pixels);
            let more_than_thresh = histogram.get_values_over_threshold_inclusive(threshold);
            let (weight_f, variance_f) = process(&more_than_thresh, number_pixels);
            let within_class_variance = (weight_b*variance_b) + (weight_f * variance_f);
            within_class_variances.push(within_class_variance);
        }
        let (mut min, mut idx): (f32, usize) = (100.0, 0);
        for index in 0..within_class_variances.len(){
            let value = within_class_variances[index];
            if value.is_nan(){
                continue;
            }
            if value < min{            
                min = value;
                idx = index;                
            }
        }
       (min, idx)
    }
}

#[cfg(test)]
mod tests {
	extern crate image;
    use im_props::histogram::Histogram;
	use super::image::*;
    use super::OtsuThreshold;
	
	#[test]
	fn otsu_test(){
        let luma: DynamicImage = testimage();
        let (variance, thresh)  = luma.get_otsu();
        assert_eq!(thresh, 3);
        assert_eq!(variance, 0.4908841);                
    }
    
    fn testimage() -> DynamicImage{
        //from http://www.labbookpages.co.uk/software/imgProc/otsuThreshold.html
        let height = 6;
        let width = 6;
		let mut luma: DynamicImage =  DynamicImage::new_luma8(width, height);
        if let DynamicImage::ImageLuma8(ref mut luma) = luma {
            for row in 0..width{
                let row = row as u32;
                if row == 0{
                    luma.get_pixel_mut(row, 0).data = [0];
                    luma.get_pixel_mut(row, 1).data = [0];
                    luma.get_pixel_mut(row, 2).data = [1];
                    luma.get_pixel_mut(row, 3).data = [4];
                    luma.get_pixel_mut(row, 4).data = [4];
                    luma.get_pixel_mut(row, 5).data = [5];                
                }
                if row == 1{
                    luma.get_pixel_mut(row, 0).data = [0];
                    luma.get_pixel_mut(row, 1).data = [1];
                    luma.get_pixel_mut(row, 2).data = [4];
                    luma.get_pixel_mut(row, 3).data = [3];
                    luma.get_pixel_mut(row, 4).data = [4];
                    luma.get_pixel_mut(row, 5).data = [4];                
                }
                if row == 2{
                    luma.get_pixel_mut(row, 0).data = [1];
                    luma.get_pixel_mut(row, 1).data = [4];
                    luma.get_pixel_mut(row, 2).data = [3];
                    luma.get_pixel_mut(row, 3).data = [2];
                    luma.get_pixel_mut(row, 4).data = [1];
                    luma.get_pixel_mut(row, 5).data = [4];                
                }
                if row == 3{
                    luma.get_pixel_mut(row, 0).data = [3];
                    luma.get_pixel_mut(row, 1).data = [3];
                    luma.get_pixel_mut(row, 2).data = [4];
                    luma.get_pixel_mut(row, 3).data = [1];
                    luma.get_pixel_mut(row, 4).data = [0];
                    luma.get_pixel_mut(row, 5).data = [0];                
                }
                if row == 4{
                    luma.get_pixel_mut(row, 0).data = [5];
                    luma.get_pixel_mut(row, 1).data = [3];
                    luma.get_pixel_mut(row, 2).data = [2];
                    luma.get_pixel_mut(row, 3).data = [1];
                    luma.get_pixel_mut(row, 4).data = [0];
                    luma.get_pixel_mut(row, 5).data = [0];                
                }
                if row == 5{
                    luma.get_pixel_mut(row, 0).data = [5];
                    luma.get_pixel_mut(row, 1).data = [5];
                    luma.get_pixel_mut(row, 2).data = [3];
                    luma.get_pixel_mut(row, 3).data = [4];
                    luma.get_pixel_mut(row, 4).data = [1];
                    luma.get_pixel_mut(row, 5).data = [0];                
                }                
            }
        }
        luma
    }    
}