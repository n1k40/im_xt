extern crate image;

use self::image::DynamicImage;
use std::collections::BTreeMap;

pub struct Histogram{
	_hashmap: BTreeMap<u8, i32>,
}

impl Histogram {
	pub fn new(image : &DynamicImage) -> Histogram{
        let mut pixel_count = BTreeMap::new();
        for key in 0..255{
            pixel_count.insert(key, 0);
        }
		for pixel in image.raw_pixels(){			
			match pixel_count.get_mut(&pixel){
				Some(count) => *count+=1,
				None => continue,
			};
			//let v = pixel_count.get(&pixel);	
			//println!("pixel is: {:?} count is: {:?}", pixel, v);			
		}
		Histogram{_hashmap:pixel_count} 
	}
	
	pub fn get_mode(&self) -> u8{
		//for (xx, yy) in &self._hashmap{
			//println!("key is : {:?} value is: {:?}", xx, yy);
		//}		
        match self._hashmap.iter().max_by(|&(_, v)| v) {
            Some((key, _)) => *key,
            None => panic!("nothing found in histogram!"),
        }
	}
    
    pub fn get_values_under_threshold(&self, threshold : usize) -> Vec<(&u8, &i32)> {
        self._hashmap.iter()
                .take(threshold)
                .collect()                       
    }	
}

#[cfg(test)]
mod tests {
	extern crate image;
    use super::Histogram;
	use super::image::{ImageBuffer, Luma, DynamicImage, GenericImage};
	
	#[test]
	fn get_histogram(){
        let height = 512;
        let width = 512;
		let mut image_buffer : ImageBuffer<Luma<u8>,_>   = ImageBuffer::new(512, 512);
		let mut luma =  DynamicImage::new_luma8(width, height);
        if let DynamicImage::ImageLuma8(ref mut luma) = luma {
            for row in 0..width{
                for column in 0..height{
                    luma.get_pixel_mut(row, column).data = [100];
                }
            }
        }
		
		let hist = Histogram::new(&luma);
		let mode = hist.get_mode();		
		println!("mode is: {:?}", mode);
		assert_eq!(100, mode);
	}
    #[test]
	fn values_under_threshold(){
        let height = 512;
        let width = 512;
		let mut luma =  DynamicImage::new_luma8(width, height);
        if let DynamicImage::ImageLuma8(ref mut luma) = luma {
            for row in 0..width{
                for column in 0..height{
                    luma.get_pixel_mut(row, column).data = [100];
                }
            }
        }
		
		let hist = Histogram::new(&luma);
        let threshold = 10;
        let x = hist.get_values_under_threshold(threshold);
        assert_eq!(threshold, x.len());
        
    }
}