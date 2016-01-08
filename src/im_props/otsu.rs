extern crate image;

mod histogram;
trait otsu_threshold{
    fn get_otsu(&self) -> i32;
}

impl otsu_threshold for image::DynamicImage{
    fn get_otsu(&self) -> i32{
        return 1;
    }
}
