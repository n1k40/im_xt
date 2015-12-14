extern crate image;

use histogram::Histogram;

use self::image::DynamicImage;

pub trait ModalPixel{
	fn get_modal_pixel(&self) -> i32;
}

impl ModalPixel for DynamicImage{
	fn get_modal_pixel(&self) -> i32{
		let pixel_count = self.get_histogram();		
		let (key, _) = pixel_count.iter().max_by(|&(_, v)| v).unwrap();
		return *key as i32;
	}
}