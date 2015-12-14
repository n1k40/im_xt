extern crate image;

use self::image::DynamicImage;
use std::collections::HashMap;

pub trait Histogram<K, V> {
	fn get_histogram(&self) -> HashMap<K, V>;
}

impl Histogram<u8, i32> for DynamicImage{
	fn get_histogram(&self) -> HashMap<u8, i32>{
		let mut pixel_count = HashMap::new();
		'outer : for pixel in self.raw_pixels(){
			if !pixel_count.contains_key(&pixel){
				pixel_count.insert(pixel, 1);
				continue 'outer;
			}
			match pixel_count.get_mut(&pixel){
				Some(count) => *count+=1,
				None => continue 'outer,
			}			
		}
		return pixel_count;
	}
	
}