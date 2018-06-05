extern crate image;

// use image::read::image::GenericImage;

pub fn read(file: &str) -> image::DynamicImage {
    return image::open(file).unwrap();
}
