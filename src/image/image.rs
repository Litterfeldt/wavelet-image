use std::simd::{f32x4};
// use std::num::Float;

pub struct Image<T> {
    width: u32,
    height: u32,
    pixels: Vec<T>
}

pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub struct YCbCr {
    y: u8,
    cb: u8,
    cr: u8
}

fn clamp(val: f32) -> u8 {
    if val < 0.0 { 0 }
    else if val > 255.0 { 255 }
    else { val.round() as u8 }
}

fn sum_f32x4(v: f32x4) -> f32 {
    v.0 + v.1 + v.2 + v.3
}

trait Convert<ColorSpace=Self> {
    type TargetColorSpace;

    fn convert(self, color: ColorSpace) -> Self::TargetColorSpace;
}

impl Convert<RGBA> for YCbCr {
    type TargetColorSpace = YCbCr;

    fn convert(self, rgba: RGBA) -> YCbCr {
        let rgb = f32x4(rgba.r as f32, rgba.g as f32, rgba.b as f32, 1.0);

        YCbCr {
            y: clamp(sum_f32x4(rgb * f32x4.new(0.299000, 0.587000, 0.114000, 0.0))),
            cb: clamp(sum_f32x4(rgb * f32x4(-0.168736, -0.331264, 0.500000, 128.0))),
            cr: clamp(sum_f32x4(rgb * f32x4(0.500000, -0.418688, -0.081312, 128.0)))
        }
    }
}
