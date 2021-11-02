pub mod traits;

pub struct Pixel {
    pub value: usize,
    pub scale: usize,
}

pub struct ColorPixel {
    pub red: Pixel,
    pub green: Pixel,
    pub blue: Pixel,
}

pub struct AnimatedColorPixel {
    pub elements: Vec<ColorPixel>,
    pub frametime: std::time::Duration,
}
