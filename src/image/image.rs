// use std::num::Float;
use image::color::Convert;

pub struct Image<T> {
    width: u32,
    height: u32,
    pixels: Vec<T>
}


impl<T: Convert<U>, U: Convert<U>> Convert<Image<T>> for Image<U> {
    type TargetColorSpace = Image<U>;

    fn convert(image: Image<T>) -> Image<U> {
        Image::<U>{
            width: image.width,
            height: image.height,
            pixels: image.pixels.into_iter().map(|p| U::convert(p)).collect()
        }
    }
}
// impl Convert<Image<RGBA>> for Image<YCbCr> {
    // type TargetColorSpace = Image<YCbCr>;

    // fn convert(image: Image<RGBA>) -> Image<YCbCr> {
        // Image::<YCbCr>{
            // width: image.width,
            // height: image.height,
            // pixels: image.pixels.into_iter().map(|p| YCbCr::convert(p)).collect()
        // }
    // }
// }

// impl Convert<Image<YCbCr>> for Image<RGBA> {
    // type TargetColorSpace = Image<RGBA>;

    // fn convert(image: Image<YCbCr>) -> Image<RGBA> {
        // Image::<RGBA>{
            // width: image.width,
            // height: image.height,
            // pixels: image.pixels.into_iter().map(|p| RGBA::convert(p)).collect()
        // }
    // }
// }
