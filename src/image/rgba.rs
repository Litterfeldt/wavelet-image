pub struct RGBAImage {
    pixels: Vec<RGBAPixel>,
    height: u32,
    width: u32,
}

pub struct RGBAPixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    x: u32,
    y: u32
}
