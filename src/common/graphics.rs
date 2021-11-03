pub struct Pixel {
    pub value: usize,
    pub scale: std::ops::Range<usize>,
}

pub struct ColorPixel {
    pub red: Pixel,
    pub green: Pixel,
    pub blue: Pixel,
}
