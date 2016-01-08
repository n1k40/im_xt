extern crate image;

use im_props::histogram::Histogram;
trait otsu_threshold{
    fn get_otsu(&self) -> i32;
}

impl otsu_threshold for image::DynamicImage{
    fn get_otsu(&self) -> i32{
        let histogram = Histogram::new(self);
        return 1;
    }
}
