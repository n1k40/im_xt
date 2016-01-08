extern crate image;

use im_props::histogram::Histogram;
trait otsu_threshold{
    fn get_otsu(&self) -> i32;
}

impl otsu_threshold for image::DynamicImage{
    fn get_otsu(&self) -> i32{
        let histogram = Histogram::new(self);
        for threshold in 0..256{
            //let weight = sum of values less than threshold / (h * w)
            //mean = sum of (value * count) / # values
            //variance 
            
        }
        return 1;
    }
}