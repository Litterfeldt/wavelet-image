use std::ops::Mul;

pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

pub struct YCbCr {
    y: u8,
    cb: u8,
    cr: u8,
}

pub trait Convert<ColorSpace=Self> {
    type TargetColorSpace;

    fn convert(color: ColorSpace) -> Self::TargetColorSpace;
}

pub struct f32x4(f32, f32, f32, f32);

impl<'a, 'b> Mul<&'b f32x4> for &'a f32x4 {
    type Output = f32x4;

    fn mul(self, other: &'b f32x4) -> f32x4 {
        f32x4(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}
fn clamp(val: f32) -> u8 {
    if val < 0.0 { 0 }
    else if val > 255.0 { 255 }
    else { val.round() as u8 }
}

fn sum_f32x4(v: f32x4) -> f32 {
    v.0 + v.1 + v.2 + v.3
}

impl Convert<RGBA> for RGB {
    type TargetColorSpace = RGB;

    fn convert(rgba: RGBA) -> RGB {
        RGB{r: rgba.r, g: rgba.g, b: rgba.b}
    }
}

impl Convert<RGB> for RGBA {
    type TargetColorSpace = RGBA;

    fn convert(rgb: RGB) -> RGBA {
        RGBA{r: rgb.r, g: rgb.g, b: rgb.b, a: 0}
    }
}

impl Convert<RGB> for YCbCr {
    type TargetColorSpace = YCbCr;

    fn convert(rgb: RGB) -> YCbCr {
        let rgb = f32x4(rgb.r as f32, rgb.g as f32, rgb.b as f32, 1.0);

        YCbCr {
            y:  clamp(sum_f32x4(&rgb * &f32x4(0.299000, 0.587000, 0.114000, 0.0))),
            cb: clamp(sum_f32x4(&rgb * &f32x4(-0.168736, -0.331264, 0.500000, 128.0))),
            cr: clamp(sum_f32x4(&rgb * &f32x4(0.500000, -0.418688, -0.081312, 128.0)))
        }
    }
}

impl Convert<RGBA> for YCbCr {
    type TargetColorSpace = YCbCr;

    fn convert(rgba: RGBA) -> YCbCr {
        YCbCr::convert(RGB::convert(rgba))
    }
}

impl Convert<YCbCr> for RGB {
    type TargetColorSpace = RGB;

    fn convert(yuv: YCbCr) -> RGB {
        let yuv = f32x4(yuv.y as f32, yuv.cb as f32  - 128.0f32, yuv.cr as f32  - 128.0f32, 0.0);

        RGB {
            r: clamp(sum_f32x4(&yuv * &f32x4(1.0,  0.0,  1.402, 0.0))),
            g: clamp(sum_f32x4(&yuv * &f32x4(1.0, -0.34414, -0.71414, 0.0))),
            b: clamp(sum_f32x4(&yuv * &f32x4(1.0,  1.772,  0.0, 0.0)))
        }
    }
}

impl Convert<YCbCr> for RGBA {
    type TargetColorSpace = RGBA;

    fn convert(yuv: YCbCr) -> RGBA {
        RGBA::convert(RGB::convert(yuv))
    }
}
