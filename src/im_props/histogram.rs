extern crate image;

use self::image::DynamicImage;
use std::collections::HashMap;

pub struct Histogram{
	_hashmap: HashMap<u8, i32>,
}

impl Histogram {
	pub fn new(image : &DynamicImage) -> Histogram{
		let mut pixel_count = HashMap::new();
		'outer : for pixel in image.raw_pixels(){
			if !pixel_count.contains_key(&pixel){
				pixel_count.insert(pixel, 1);
				continue 'outer;
			}
			match pixel_count.get_mut(&pixel){
				Some(count) => *count+=1,
				None => continue 'outer,
			}			
		}
		return Histogram{_hashmap:pixel_count} 
	}
	
	pub fn get_mode(&self) -> u8{
		let (key, _) = self._hashmap.iter().max_by(|&(_, v)| v).unwrap();
 		return *key;
	}	
}

#[cfg(test)]
mod tests {
	extern crate image;
    use super::Histogram;
	use super::image::{ImageBuffer, Luma};
	
	#[test]
	fn get_histogram(){
		let img : ImageBuffer<Luma<u8>,_>   = ImageBuffer::new(512, 512);
		assert!(true);
	}
}